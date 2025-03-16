use std::io::{self, BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;

pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("192.168.1.52:8080")?;
    println!("Server running on 192.168.1.52:8080");

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
                if let Err(e) = handle_client(&mut stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    println!("Server stopped.");
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

