use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// Handle an individual client connection.
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // Read from the connection. In a real application, you’d want to loop until you've read the entire request.
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Received {} bytes:", bytes_read);
            println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
        }
        Err(e) => eprintln!("Failed to read from connection: {}", e),
    }

    // A very simple HTTP response.
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!\r\n";
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to connection: {}", e);
    }
    // Flush the stream to ensure the response is sent immediately.
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
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
                handle_client(stream);
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
