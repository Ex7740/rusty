use std::io::{self, Write};
use std::net::TcpStream;

/// Connects to a server at the given address and allows continuous messaging
pub fn connect_to_server(address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Successfully connected to {}", address);

    let stdin = io::stdin();
    loop {
        let mut message = String::new();
        println!("Enter your message (Ctrl+C to exit):");
        stdin.read_line(&mut message).expect("Failed to read input");

        stream.write_all(message.trim().as_bytes())?;
        println!("Message sent: {}", message.trim());
    }
}

