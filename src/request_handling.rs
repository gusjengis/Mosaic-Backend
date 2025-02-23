use crate::modify::clear_tests;
use crate::{export::*, import::fetch_data};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn process_request(stream: &mut TcpStream, header: String, body: String) {
    let mut iter = header.split(" ");
    let _method = iter.next().unwrap().to_string();
    let endpoint = iter.next().unwrap().to_string();
    let mut response;
    match endpoint.as_str() {
        "/logUpload" => {
            forward_data("logUpload", body);
            response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }
        "/logDownload" => {
            let res_body = fetch_data("logDownload", body).await;
            let content_length = res_body.len();
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                content_length, res_body
            );
        }
        "/testDBConnection" => {
            fetch_data("testDBConnection", body).await;
            response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }
        "/clearTests" => {
            clear_tests().await;
            response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }
        _ => {
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}{}\r\n",
                header, body,
            );
        }
    }

    if let Err(e) = stream.write_all(response.as_bytes()).await {
        eprintln!("Failed to write to connection: {}", e);
    }

    if let Err(e) = stream.flush().await {
        eprintln!("Failed to flush connection: {}", e);
    }
}
