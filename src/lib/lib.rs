#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! This is the library part of the `simpdiscovery` crate for simple UDP datagram-based discovery
//! of services on a Local Area Network
//!
/// # Example Usage in a combined BeaconSender and BeaconListener
/// ```
/// use simpdiscoverylib::{BeaconSender, BeaconListener};
/// use std::time::Duration;
///
/// let port = 34254;
/// let my_service_name = "net.mackenzie-serres.simpdiscovery";
/// if let Ok(beacon) = BeaconSender::new(port, my_service_name) {
///     std::thread::spawn(move || {
///         let _ = beacon.send_loop(Duration::from_secs(1));
///     });
/// }
///
/// let listener = BeaconListener::new(port, None).expect("Could not create listener");
/// let beacon = listener.wait(None).expect("Failed to receive beacon");
/// assert_eq!(beacon.message, my_service_name, "Service name received in beacon doesn't match the one expected");
/// ```

use std::net::UdpSocket;
use std::time::Duration;
use log::info;

//const BROADCAST_ADDRESS : &str = "192.168.2.255";
const BROADCAST_ADDRESS : &str = "255.255.255.255";
const MAX_INCOMING_BEACON_SIZE : usize = 1024;

/// `BeaconSender` is used to send UDP Datagram beacons to the Broadcast IP address on the LAN
pub struct BeaconSender {
    socket: UdpSocket,
    broadcast_address: String,
    message: Vec<u8>,
}

/// # Example of a BeaconSender
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
/// let port = 34254;
/// if let Ok(beacon) = BeaconSender::new(port, "Hello") {
///     std::thread::spawn(move || {
///         let _ = beacon.send_loop(Duration::from_secs(1));
///     });
/// }
impl BeaconSender {
    /// Create a new `BeaconSender` setup to send `Beacon`s on the specified `port`
    pub fn new(port: u16, service_name: &str) -> std::io::Result<Self> {
        // Setting the port to non-zero (or at least the same port used in listener) causes
        // this to fail. I am not sure of the correct value to use. Docs on UDP says '0' is
        // permitted, if you do not expect a response from the UDP Datagram sent.
        let bind_address = "0.0.0.0:0";
        let socket:UdpSocket = UdpSocket::bind(bind_address)?;
        info!("Socket bound to: {}", bind_address);

        socket.set_broadcast(true)?;
        info!("Broadcast mode set to ON");

        Ok(Self {
            socket,
            broadcast_address: format!("{}:{}", BROADCAST_ADDRESS, port),
            message : service_name.as_bytes().to_vec()
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
        info!("Sending Beacon to: '{}'", self.broadcast_address);
        self.socket.send_to(&self.message, &self.broadcast_address)
    }
}

/// `Beacon` contains information about the beacon that was received by a `BeaconListener`
pub struct Beacon {
    /// The IP address and port the beacon was sent from
    pub source_ip: String,
    /// The port the beacon came from
    pub source_port: u16,
    /// The message included in the beacon
    pub message: String
}

/// `BeaconListener` listens for new `Beacons` on the specified port
pub struct BeaconListener {
    socket: UdpSocket,
    filter: Option<String>
}

impl BeaconListener {
    /// Create a new `BeaconListener` on the specified port
    pub fn new(port: u16, filter: Option<String>) -> std::io::Result<Self> {
        let address = format!("{}:{}", "0.0.0.0", port);
        let socket = UdpSocket::bind(&address)?;
        info!("Socket bound to: {}", address);

        Ok(Self {
            socket,
            filter
        })
    }

    /// Wait for a `Beacon` on the port specified in `BeaconListener::new()`
    ///
    /// # Example with timeout
    ///
    /// ```
    /// use simpdiscoverylib::BeaconListener;
    /// use std::time::Duration;
    ///
    /// let port = 34254;
    /// let listener = BeaconListener::new(port, None).expect("Could not create listener");
    /// let beacon = listener.wait(Some(Duration::from_millis(1)));
    /// assert!(beacon.is_err());
    /// ```
    pub fn wait(&self, timeout: Option<Duration>) -> std::io::Result<Beacon> {
        let mut buffer = [0; MAX_INCOMING_BEACON_SIZE];

        self.socket.set_read_timeout(timeout)?;
        info!("Read timeout set to: {:?}", timeout);

        info!("Waiting for beacon");
        loop {
            let (number_of_bytes, source_address) = self.socket.recv_from(&mut buffer)?;
            let message = String::from_utf8(buffer[..number_of_bytes].to_vec())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other,
                                                 e.to_string())
                )?;
            info!("Message '{}' received from IP: '{}' on port: '{}'", message, source_address.ip(), source_address.port());

            match &self.filter {
                Some(match_string) => {
                    if &message == match_string {
                        return Ok(Beacon {
                            source_ip: source_address.ip().to_string(),
                            source_port: source_address.port(),
                            message
                        });
                    }
                },
                None => return Ok(Beacon {
                    source_ip: source_address.ip().to_string(),
                    source_port: source_address.port(),
                    message
                })
            }
        }
    }
}
