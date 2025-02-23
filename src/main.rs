mod db;
mod deserialization;
mod export;
mod import;
mod log;
mod modify;
mod request_collection;
mod request_handling;

use dotenvy::dotenv;
use request_collection::collect_http_request;
use request_handling::process_request;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Optionally load environment variables.
    dotenv().ok();

    let port = 8080;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server listening on port {}", port);

    loop {
        let (mut socket, _) = listener.accept().await?;
        // Spawn a new asynchronous task for each connection.
        tokio::spawn(async move {
            match collect_http_request(&mut socket).await {
                Ok((header, body)) => {
                    process_request(
                        &mut socket,
                        String::from_utf8_lossy(&header).to_string(),
                        String::from_utf8_lossy(&body).to_string(),
                    )
                    .await;
                }
                Err(e) => {
                    eprintln!("Error collecting HTTP request: {}", e);
                }
            }
        });
    }
}
