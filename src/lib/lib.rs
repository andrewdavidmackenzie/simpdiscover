use std::net::UdpSocket;
use std::time::Duration;
use log::info;

//const BROADCAST_ADDRESS : &str = "192.168.2.255";
const BROADCAST_ADDRESS : &str = "255.255.255.255";

pub fn beacon(port: usize) -> std::io::Result<()> {
    let bind_address = "0.0.0.0:0";
    let socket:UdpSocket = UdpSocket::bind(bind_address)?;
    info!("Socket bound to: {}", bind_address);

    socket.set_broadcast(true)?;
    info!("Broadcast mode set to ON");

    let message = "Hello";
    let broadcast_address = format!("{}:{}", BROADCAST_ADDRESS, port);
    socket.send_to(message.as_bytes(), &broadcast_address)?;

    loop {
        info!("Sending Beacon to: '{}'", broadcast_address);
        socket.send_to(message.as_bytes(), &broadcast_address)?;
        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn beacon_listener(port: usize) -> std::io::Result<String> {
    let address = format!("{}:{}", "0.0.0.0", port);
    let socket = UdpSocket::bind(&address)?;
    info!("Socket bound to: {}", address);

    // Receives a single datagram message on the socket.
    let mut buffer = [0; 10];

    info!("Listening on: '{}'", address);
    let (_number_of_bytes, source_address) = socket.recv_from(&mut buffer)?;

    info!("Message '{}' received from Address: '{}'", String::from_utf8(Vec::from(buffer)).unwrap(), source_address);

    Ok(source_address.to_string())
}