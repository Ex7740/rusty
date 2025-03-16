
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

/// Starts the server and listens for connections
pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on 127.0.0.1:8080 \n Press Ctrl+c to exit server");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected: {}", stream.peer_addr()?);
                handle_client(&mut stream)?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(());
    }

    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    stream.write_all(b"Message received")?;
    Ok(())
}

