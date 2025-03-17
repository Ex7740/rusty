use std::io::{self, BufRead, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::io::ErrorKind;

fn get_local_ip() -> io::Result<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?.ip().to_string())
}

pub fn start_server() -> io::Result<()> {
    let local_ip = get_local_ip()?;
    let address = format!("{}:8080", local_ip);
    let listener = TcpListener::bind(&address)?;
    println!("Server running on {}", address);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Thread for handling admin commands
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line.unwrap().trim() {
                "exit" => {
                    println!("Shutting down server...");
                    running_clone.store(false, Ordering::SeqCst);
                    std::process::exit(0);
                }
                "status" => {
                    println!("Server is running and accepting connections.");
                }
                "help" => {
                    println!("Available commands:");
                    println!("  exit   - Shut down the server");
                    println!("  status - Show server status");
                    println!("  help   - List available commands");
                }
                command => println!("Unknown command: '{}'. Type 'help' for a list of commands.", command),
            }
        }
    });

    // Main loop for accepting connections
    for stream in listener.incoming() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        match stream {
            Ok(mut stream) => {
                println!("Client connected: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    if let Err(e) = handle_client(&mut stream) {
                        if e.kind() != ErrorKind::ConnectionReset {
                            eprintln!("Error handling client: {}", e);
                        } else {
                            println!("Client disconnected gracefully.");
                        }
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    println!("Server stopped.");
    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Client disconnected
            Ok(bytes_read) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                stream.write_all(b"Message received")?;
            }
            Err(e) if e.kind() == ErrorKind::ConnectionReset => break, // Handle client disconnection
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

