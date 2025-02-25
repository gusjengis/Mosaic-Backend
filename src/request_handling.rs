use crate::endpoints::log_delete::log_delete;
use crate::endpoints::log_load::log_load;
use crate::endpoints::log_load_range::log_load_range;
use crate::endpoints::log_upload::log_upload;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_http_request(stream: &mut TcpStream, header: String, body: String) {
    let mut iter = header.split(" ");
    let _method = iter.next().unwrap().to_string();
    let endpoint = iter.next().unwrap().to_string();
    let response = match endpoint.as_str() {
        "/logUpload" => log_upload(body).await,
        "/logLoad" => log_load().await,
        "/logLoadRange" => log_load_range(body).await,
        "/logDelete" => log_delete(body).await,
        _ => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}{}\r\n",
            header, body
        ),
    };

    if let Err(e) = stream.write_all(response.as_bytes()).await {
        eprintln!("Failed to write to connection: {}", e);
    }

    if let Err(e) = stream.flush().await {
        eprintln!("Failed to flush connection: {}", e);
    }
}
