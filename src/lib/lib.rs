#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! This is the library part of the `simpdiscovery` crate for simple UDP datagram-based discovery
//! of services on a Local Area Network

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
    /// Create a new `BeaconSender` setup to send `Beacon`s on the specified `port`
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

    /// Enter an infinite loop sending `Beacon`s periodically
    pub fn send_loop(&self) -> std::io::Result<()> {
        loop {
            self.send_one_beacon()?;
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    /// Send a single `Beacon` out
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

/// `BeaconListener` listens for new `Beacons` on the specified port
pub struct BeaconListener {
    socket: UdpSocket,
}

impl BeaconListener {
    /// Create a new `BeaconListener` on the specified port
    pub fn new(port: usize) -> std::io::Result<Self> {
        let address = format!("{}:{}", "0.0.0.0", port);
        let socket = UdpSocket::bind(&address)?;
        info!("Socket bound to: {}", address);

        Ok(Self {
            socket
        })
    }

    /// Wait for a `Beacon` on the port specified in `BeaconListener::new()`
    pub fn wait(&self) -> std::io::Result<Beacon> {
        let mut buffer = [0; 5]; // TODO

        info!("Waiting for beacon");
        let (_number_of_bytes, source_address) = self.socket.recv_from(&mut buffer)?;
        let message = String::from_utf8(Vec::from(buffer)).unwrap();
        info!("Message '{}' received from Address: '{}'", message, source_address);

        Ok(Beacon{
            source_ip: source_address.to_string(),
            message
        })
    }
}
