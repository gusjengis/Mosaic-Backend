use std::{io::Write, net::TcpStream};

pub fn process_request(stream: &mut TcpStream, request: String) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!\n\n{}\r\n",
        request
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to connection: {}", e);
    }

    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}
