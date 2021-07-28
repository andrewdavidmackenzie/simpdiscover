#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! This is the `simpdiscoverylib` create for simple UDP databagram based discovery of services
//! on a LAN

use std::net::UdpSocket;
use std::time::Duration;
use log::info;


//const BROADCAST_ADDRESS : &str = "192.168.2.255";
const BROADCAST_ADDRESS : &str = "255.255.255.255";

/// `BeaconSender` is used to send UDP Datagram beacons to the Broadcast IP address on the LAN
pub struct BeaconSender {
    socket: UdpSocket,
    broadcast_address: String,
    message: &'static [u8],
}

impl BeaconSender {
    /// Create a new `Beacon` setup to send beacons on the specified `port`
    pub fn new(port: usize) -> std::io::Result<Self> {
        let bind_address = "0.0.0.0:0";
        let socket:UdpSocket = UdpSocket::bind(bind_address)?;
        info!("Socket bound to: {}", bind_address);

        socket.set_broadcast(true)?;
        info!("Broadcast mode set to ON");

        Ok(Self {
            socket,
            broadcast_address: format!("{}:{}", BROADCAST_ADDRESS, port),
            message : "Hello".as_bytes()
        })
    }

    /// Enter an infinite loop sending beacons periodically
    pub fn send_loop(&self) -> std::io::Result<()> {
        loop {
            self.send_one_beacon()?;
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    /// Send a single beacon out
    pub fn send_one_beacon(&self) -> std::io::Result<usize> {
        info!("Sending Beacon to: '{}'", self.broadcast_address);
        self.socket.send_to(self.message, &self.broadcast_address)
    }
}

/// `Beacon` contains information about the beacon that was received by a `BeaconListener`
pub struct Beacon {
    /// The IP address and port the beacon was sent from
    pub source_ip: String,
    /// The message included in the beacon
    pub message: String
}

/// Listen for a beacon on the specified port - blocking until one is received
pub fn beacon_listener(port: usize) -> std::io::Result<Beacon> {
    let address = format!("{}:{}", "0.0.0.0", port);
    let socket = UdpSocket::bind(&address)?;
    info!("Socket bound to: {}", address);

    // Receives a single datagram message on the socket.
    let mut buffer = [0; 5];

    info!("Listening on: '{}'", address);
    let (_number_of_bytes, source_address) = socket.recv_from(&mut buffer)?;
    let message = String::from_utf8(Vec::from(buffer)).unwrap();
    info!("Message '{}' received from Address: '{}'", message, source_address);

    Ok(Beacon{
        source_ip: source_address.to_string(),
        message
    })
}