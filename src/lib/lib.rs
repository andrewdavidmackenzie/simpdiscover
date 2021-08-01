#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! This is the library part of the `simpdiscovery` crate for simple UDP datagram-based discovery
//! of services on a Local Area Network
//!
//! # Example Usage in a combined BeaconSender and BeaconListener
//! ```
//! use simpdiscoverylib::{BeaconSender, BeaconListener};
//! use std::time::Duration;
//!
//! let port = 9001;
//! let my_service_name = "_my_service._tcp.local".as_bytes();
//! if let Ok(beacon) = BeaconSender::new(port, my_service_name) {
//!     std::thread::spawn(move || {
//!         let _ = beacon.send_loop(Duration::from_secs(1));
//!     });
//! }
//!
//! let listener = BeaconListener::new(my_service_name).expect("Could not create listener");
//! let beacon = listener.wait(None).expect("Failed to receive beacon");
//! assert_eq!(beacon.service_name, my_service_name, "Service name received in beacon doesn't match the one expected");
//! assert_eq!(beacon.service_port, port);
//! ```

use std::net::UdpSocket;
use std::time::Duration;
use log::{info, trace};
use std::fmt::Formatter;

//const BROADCAST_ADDRESS : &str = "192.168.2.255";
// We have chosen 9001 as the BEACON_PORT for now
const BROADCAST_ADDRESS : &str = "255.255.255.255:9001";
const LISTENING_ADDRESS : &str = "0.0.0.0:9001";
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
///
/// if let Ok(beacon) = BeaconSender::new(9001, "Hello".as_bytes()) {
///     std::thread::spawn(move || {
///         let _ = beacon.send_loop(Duration::from_secs(1));
///     });
/// }
pub struct BeaconSender {
    socket: UdpSocket,
    beacon_payload: Vec<u8>,
}

fn u16_to_array_of_u8(x:u16) -> [u8;2] {
    let b1 : u8 = ((x >> 8) & 0xff) as u8;
    let b2 : u8 = (x & 0xff) as u8;
    return [b1, b2]
}

fn array_of_u8_to_u16(array: &[u8]) -> u16 {
    let upper : u16 = (array[0] as u16) << 8;
    let lower : u16 = array[1] as u16;
    return upper + lower
}

impl BeaconSender {
    /// Create a new `BeaconSender` to send `Beacon`s for a service with name `service_name` that
    /// should be contacted on the port `service_port`
    pub fn new(service_port: u16, service_name: &[u8]) -> std::io::Result<Self> {
        // Setting the port to non-zero (or at least the same port used in listener) causes
        // this to fail. I am not sure of the correct value to use. Docs on UDP says '0' is
        // permitted, if you do not expect a response from the UDP Datagram sent.
        let bind_address = "0.0.0.0:0";
        let socket:UdpSocket = UdpSocket::bind(bind_address)?;
        info!("Socket bound to: {}", bind_address);

        socket.set_broadcast(true)?;
        info!("Broadcast mode set to ON");

        // Create payload with magic number, service_port number and service_name
        let mut beacon_payload: Vec<u8> = u16_to_array_of_u8(MAGIC_NUMBER).to_vec();
        beacon_payload.append(&mut u16_to_array_of_u8(service_port).to_vec());
        beacon_payload.append(&mut service_name.to_vec().clone());

        Ok(Self {
            socket,
            beacon_payload,
        })
    }

    /// Enter an infinite loop sending `Beacon`s periodically
    pub fn send_loop(&self, period: Duration) -> std::io::Result<()> {
        loop {
            self.send_one_beacon()?;
            std::thread::sleep(period);
        }
    }

    /// Send a single `Beacon` out
    pub fn send_one_beacon(&self) -> std::io::Result<usize> {
        println!("Sending Beacon to: '{}'", BROADCAST_ADDRESS);
        self.socket.send_to(&self.beacon_payload, BROADCAST_ADDRESS)
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
/// # Example of using `BeaconSender` with timeout
/// ```
/// use simpdiscoverylib::BeaconListener;
/// use std::time::Duration;
///
/// let port = 9001;
/// let listener = BeaconListener::new("_my_service._tcp.local".as_bytes()).expect("Could not create listener");
///
/// // Avoid blocking tests completely with no timeout, and set a very short one
/// let beacon = listener.wait(Some(Duration::from_millis(1)));
/// assert!(beacon.is_err());
/// ```
pub struct BeaconListener {
    socket: UdpSocket,
    service_name: Vec<u8>
}

impl BeaconListener {
    /// Create a new `BeaconListener` on `port` with an option `filter` to be applied to incoming
    /// beacons. This binds to address "0.0.0.0:port"
    pub fn new(service_name: &[u8]) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(&LISTENING_ADDRESS)?;
        trace!("Socket bound to: {}", LISTENING_ADDRESS);

        Ok(Self {
            socket,
            service_name: service_name.to_vec()
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
    pub fn wait(&self, timeout: Option<Duration>) -> std::io::Result<Beacon> {
        self.socket.set_read_timeout(timeout)?;
        info!("Read timeout set to: {:?}", timeout);

        info!("Waiting for beacon");
        loop {
            let beacon = self.receive_one_beacon()?;

            if beacon.service_name == self.service_name {
                trace!("Message matches filter: returning beacon");
                return Ok(beacon);
            } else {
                trace!("Message does not match filter: ignoring");
            }
        }
    }

    /*
        Receive one beacon
     */
    fn receive_one_beacon(&self) -> std::io::Result<Beacon> {
        let mut buffer = [0; MAX_INCOMING_BEACON_SIZE];

        loop {
            let (number_of_bytes, source_address) = self.socket.recv_from(&mut buffer)?;
            let magic_number = array_of_u8_to_u16(&buffer[0..2]);
            if magic_number == MAGIC_NUMBER {
                let service_port = array_of_u8_to_u16(&buffer[2..4]);
                let service_name = buffer[4..number_of_bytes].to_vec();
                trace!("Message received from IP: '{}' on port: '{}'", source_address.ip(), source_address.port());

                return Ok(Beacon {
                    service_ip: source_address.ip().to_string(),
                    service_port,
                    service_name
                });
            }
        }
    }
}
