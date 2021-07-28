use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    beacon_listener(34254)?;

    Ok(())
}

fn beacon_listener(port: usize) -> std::io::Result<()> {
    let address = format!("{}:{}", "0.0.0.0", port);
    let socket = UdpSocket::bind(&address)?;
    println!("Socket bound to: {}", address);

    // Receives a single datagram message on the socket.
    let mut buffer = [0; 10];

    println!("Listening on: '{}'", address);
    let (_number_of_bytes, source_address) = socket.recv_from(&mut buffer)?;

    println!("Message '{}' received from Address: '{}'", String::from_utf8(Vec::from(buffer)).unwrap(), source_address);

    Ok(())
}