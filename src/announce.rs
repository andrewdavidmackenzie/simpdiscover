use std::net::UdpSocket;
use std::time::Duration;

//const BROADCAST_ADDRESS : &str = "192.168.2.255";
const BROADCAST_ADDRESS : &str = "255.255.255.255";

fn main() -> std::io::Result<()> {
    beacon(34254)?;

    Ok(())
}

fn beacon(port: usize) -> std::io::Result<()> {
    let bind_address = "0.0.0.0:0";
    let socket:UdpSocket = UdpSocket::bind(bind_address)?;
    println!("Socket bound to: {}", bind_address);

    socket.set_broadcast(true)?;
    println!("Broadcast mode set to ON");

    let message = "Hello";
    let broadcast_address = format!("{}:{}", BROADCAST_ADDRESS, port);
    socket.send_to(message.as_bytes(), &broadcast_address)?;

    loop {
        println!("Sending Beacon to: '{}'", broadcast_address);
        socket.send_to(message.as_bytes(), &broadcast_address)?;
        std::thread::sleep(Duration::from_secs(1));
    }
}