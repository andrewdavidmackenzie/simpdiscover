#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! `simpdiscovery` library crate for simple UDP datagram-based discovery of services on a LAN
//!
//! # Example combining a BeaconSender and a BeaconListener
//! ```
//! use simpdiscoverylib::{BeaconSender, BeaconListener};
//! use std::time::Duration;
//! use portpicker::pick_unused_port;
//!
//! let service_port = pick_unused_port().expect("Could not get a free port");
//! let broadcast_port = pick_unused_port().expect("Could not get a free port");
//! let my_service_name = "_my_service._tcp.local".as_bytes();
//! let beacon = BeaconSender::new(service_port, my_service_name, broadcast_port)
//!     .expect("Could not create sender");
//! std::thread::spawn(move || {
//!     beacon.send_loop(Duration::from_secs(1)).expect("Could not run send_loop")
//! });
//!
//! let listener = BeaconListener::new(my_service_name, broadcast_port)
//!     .expect("Could not create listener");
//! let beacon = listener.wait(None).expect("Failed to receive beacon");
//! assert_eq!(beacon.service_name, my_service_name, "Received service name doesn't match");
//! assert_eq!(beacon.service_port, service_port, "Received service port doesn't match");
//! ```

use std::net::UdpSocket;
use std::time::Duration;
use log::{info, trace};
use std::fmt::Formatter;
use std::io;

/// A broadcast address is always relative to a given network. When you have a network, you can
/// compute its broadcast address by replacing all the host bits with 1s; simply put, the broadcast
/// address is the highest numbered address you can have on the network, while the network address
/// is the lowest one (with all host bits set to 0s); this is why you can't use either of them
/// as actual host addresses: they are reserved for this use.
///
/// If your network is `192.168.1.0/24`, then your network address will be `192.168.1.0`
/// and your broadcast address will be `192.168.1.255`
///
/// If your network is `192.168.0.0/16`, then your network address will be `192.168.0.0`
/// and your broadcast address will be `192.168.255.255`
///
/// `255.255.255.255` is a special broadcast address, which means "this network".
/// It lets you send a broadcast packet to the network you're connected to, without actually
/// caring about its address.
///
/// See [wikipedia article](https://en.wikipedia.org/wiki/Broadcast_address) for more info
const BROADCAST_ADDRESS : &str = "255.255.255.255";

/// The address `0.0.0.0` is known as the "zero network", which in Internet Protocol standards
/// stands for this network, i.e. the local network.
const LISTENING_ADDRESS : &str = "0.0.0.0";

const MAX_INCOMING_BEACON_SIZE : usize = 1024;
const MAGIC_NUMBER: u16 = 0xbeef;

/// `BeaconSender` is used to send UDP Datagram beacons to the Broadcast IP address on the LAN
///
/// # Example of using `BeaconSender`
/// This example will just exit at the end and the thread above will die along with the process.
///
/// In your own code, either:
///   * don't start a background thread and just loop forever sending beacons in main thread, or
///   * have some other way to keep the process (and hence the sending thread) alive so
///     beacons are actually sent before process ends
///
/// ```
/// use simpdiscoverylib::{BeaconSender, BeaconListener};
/// use std::time::Duration;
/// use portpicker::pick_unused_port;
///
/// let service_port = pick_unused_port().expect("Could not get a free port");
/// let broadcast_port = pick_unused_port().expect("Could not get a free port for broadcast");
/// let my_service_name = "_my_service._tcp.local".as_bytes();
/// let beacon = BeaconSender::new(service_port, my_service_name, broadcast_port)
///     .expect("Could not create sender");
/// std::thread::spawn(move || {
///     beacon.send_loop(Duration::from_secs(1)).expect("Could not enter send_loop");
///  });
pub struct BeaconSender {
    socket: UdpSocket,
    beacon_payload: Vec<u8>,
    broadcast_address: String,
}

fn u16_to_array_of_u8(x:u16) -> [u8;2] {
    let b1 : u8 = ((x >> 8) & 0xff) as u8;
    let b2 : u8 = (x & 0xff) as u8;
    [b1, b2]
}

fn array_of_u8_to_u16(array: &[u8]) -> u16 {
    let upper : u16 = (array[0] as u16) << 8;
    let lower : u16 = array[1] as u16;
    upper + lower
}

impl BeaconSender {
    /// Create a new `BeaconSender` to send `Beacon`s for a service with name `service_name` that
    /// should be contacted on the port `service_port`
    pub fn new(service_port: u16, service_name: &[u8], broadcast_port: u16) -> io::Result<Self> {
        // Setting the port to non-zero (or at least the same port used in listener) causes
        // this to fail. I am not sure of the correct value to use. Docs on UDP says '0' is
        // permitted, if you do not expect a response from the UDP Datagram sent.
        let bind_address = format!("{LISTENING_ADDRESS}:0");
        let socket:UdpSocket = UdpSocket::bind(&bind_address)
            .map_err(|e|
                         io::Error::new(io::ErrorKind::AddrInUse,
                                        format!("SimpDiscover::BeaconSender could not bind to UdpSocket {bind_address} ({e})")))?;
        info!("Socket bound to: {}", bind_address);

        socket.set_broadcast(true)?;
        info!("Broadcast mode set to ON");

        // Create payload with magic number, service_port number and service_name
        let mut beacon_payload: Vec<u8> = u16_to_array_of_u8(MAGIC_NUMBER).to_vec();
        beacon_payload.append(&mut u16_to_array_of_u8(service_port).to_vec());
        beacon_payload.append(&mut service_name.to_vec());

        let broadcast_address = format!("{BROADCAST_ADDRESS}:{broadcast_port}");

        Ok(Self {
            socket,
            beacon_payload,
            broadcast_address,
        })
    }

    /// Enter an infinite loop sending `Beacon`s periodically
    pub fn send_loop(&self, period: Duration) -> io::Result<()> {
        loop {
            self.send_one_beacon()?;
            std::thread::sleep(period);
        }
    }

    /// Send a single `Beacon` out
    pub fn send_one_beacon(&self) -> io::Result<usize> {
        trace!("Sending Beacon '{}' to: '{}'", String::from_utf8_lossy(&self.beacon_payload[4..]),
            self.broadcast_address);
        self.socket.send_to(&self.beacon_payload, &self.broadcast_address)
    }
}

/// `Beacon` contains information about the beacon that was received by a `BeaconListener`
pub struct Beacon {
    /// The IP address and port the beacon was sent from
    pub service_ip: String,
    /// The port the service is running on
    pub service_port: u16,
    /// The name of the service sending the beacon
    pub service_name: Vec<u8>
}

impl std::fmt::Display for Beacon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let service_name = String::from_utf8(self.service_name.clone()).unwrap_or_else(|_| "Invalid UTF-8 String".into());
        write!(f, "ServiceName: '{}', Service IP: {}, Service Port: {}", service_name, self.service_ip, self.service_port)
    }
}

/// `BeaconListener` listens for new `Beacons` on the specified port
///
/// # Example of using `BeaconListener` with timeout
/// ```
/// use simpdiscoverylib::BeaconListener;
/// use std::time::Duration;
/// use portpicker::pick_unused_port;
///
/// let listening_port = pick_unused_port().expect("Could not get a free port to listen on");
/// let listener = BeaconListener::new("_my_service._tcp.local".as_bytes(), listening_port)
///     .expect("Could not create listener");
///
/// // Avoid blocking tests by setting a short timeout, expect an error, as there is no sender setup
/// assert!(listener.wait(Some(Duration::from_millis(1))).is_err());
/// ```
pub struct BeaconListener {
    socket: UdpSocket,
    service_name: Vec<u8>,
}

impl BeaconListener {
    /// Create a new `BeaconListener` on `port` with an option `filter` to be applied to incoming
    /// beacons. This binds to address "0.0.0.0:listening_port"
    pub fn new(service_name: &[u8], listening_port: u16) -> io::Result<Self> {
        let listening_address = format!("{}:{}", LISTENING_ADDRESS, listening_port);
        let socket = UdpSocket::bind(&listening_address)
            .map_err(|e|
                io::Error::new(io::ErrorKind::AddrInUse,
                               format!("SimpDiscover::BeaconListener could not bind to UdpSocket at {listening_address} ({e})")))?;
        trace!("Socket bound to: {}", listening_address);
        socket.set_broadcast(true)?;

        Ok(Self {
            socket,
            service_name: service_name.to_vec(),
        })
    }

    /// Wait for a `Beacon` on the port specified in `BeaconListener::new()`
    /// If `timeout` is None, then it will block forever waiting for a beacon matching the optional
    /// filter (if supplied) in `BeaconListener::new()`. If no `filter` was supplied it will block
    /// waiting for any beacon to be received.
    ///
    /// If `timeout` is `Some(Duration)` then it will block for that duration on the reception of
    /// each beacon. If the beacon does not match a supplied `filter` then it will loop (blocking
    /// for `duration` each time until a matching beacon is found.
    pub fn wait(&self, timeout: Option<Duration>) -> io::Result<Beacon> {
        self.socket.set_read_timeout(timeout)?;
        info!("Read timeout set to: {:?}", timeout);

        info!("Waiting for beacon matching '{}'", String::from_utf8_lossy(&self.service_name));
        loop {
            let beacon = self.receive_one_beacon()?;

            if beacon.service_name == self.service_name {
                trace!("Beacon '{}' matches filter '{}': returning beacon",
                    String::from_utf8_lossy(&beacon.service_name), String::from_utf8_lossy(&self.service_name));
                return Ok(beacon);
            } else {
                trace!("Beacon '{}' does not match filter '{}': ignoring",
                    String::from_utf8_lossy(&beacon.service_name), String::from_utf8_lossy(&self.service_name));
            }
        }
    }

    /*
        Receive one beacon
     */
    fn receive_one_beacon(&self) -> io::Result<Beacon> {
        let mut buffer = [0; MAX_INCOMING_BEACON_SIZE];

        loop {
            let (number_of_bytes, source_address) = self.socket.recv_from(&mut buffer)?;
            let magic_number = array_of_u8_to_u16(&buffer[0..2]);
            if magic_number == MAGIC_NUMBER {
                let service_port = array_of_u8_to_u16(&buffer[2..4]);
                let service_name = buffer[4..number_of_bytes].to_vec();

                return Ok(Beacon {
                    service_ip: source_address.ip().to_string(),
                    service_port,
                    service_name
                });
            }
        }
    }
}
