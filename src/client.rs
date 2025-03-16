use std::io::{self, Write};
use std::net::TcpStream;

/// Connects to a server at the given address
pub fn connect_to_server(address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Successfully connected to {}", address);

    let message = "Hello from Rusty Client!";
    stream.write_all(message.as_bytes())?;
    println!("Message sent: {}", message);

    Ok(())
}
