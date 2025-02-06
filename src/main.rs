pub mod request_collection;
pub mod request_handling;

use std::net::{TcpListener, TcpStream};
use std::thread;

use request_collection::collect_http_request;
use request_handling::process_request;

/// Handle an individual client connection.
fn handle_client(mut stream: TcpStream) {
    match collect_http_request(&mut stream) {
        Ok(buffer) => {
            println!("Received {} bytes:", buffer.len());
            let request = String::from_utf8_lossy(&buffer).to_string();
            process_request(&mut stream, request);
        }
        Err(e) => eprintln!("Failed to read from connection: {}", e),
    }
}

fn main() -> std::io::Result<()> {
    // Bind the listener to all interfaces on port 8080.
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Server listening on port 8080");

    // Accept incoming connections in a loop.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // For a production server you’d want to handle each connection concurrently,
                // e.g., by spawning a thread or using an async runtime.
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
