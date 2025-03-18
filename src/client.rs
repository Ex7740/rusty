use std::io::{self, Write};
use std::net::TcpStream;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Connects to a server at the given address and allows continuous messaging
pub fn connect_to_server(address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Successfully connected to {}", address);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nDisconnecting from the server...");
    }).expect("Error setting Ctrl-C handler");

    let stdin = io::stdin();
    
    // Prompt for username
    let mut username = String::new();
    print!("Enter your username: ");
    io::stdout().flush()?;
    stdin.read_line(&mut username)?;
    let username = username.trim().to_string();
    
    while running.load(Ordering::SeqCst) {
        let mut message = String::new();
        println!("Enter your message:");
        if stdin.read_line(&mut message).is_err() {
            println!("Failed to read input");
            continue;
        }
        let message = message.trim();
        
        let formatted_message = format!("{}: {}", username, message);
        stream.write_all(formatted_message.as_bytes())?;
        println!("Message sent: {}", formatted_message);
    }

    Ok(())
}

