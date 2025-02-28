mod db;
mod endpoints;
mod log;
mod request_collection;
mod request_handling;

use std::env;

use dotenvy::dotenv;
use once_cell::sync::Lazy;
use request_collection::collect_http_request;
use request_handling::handle_http_request;
use tokio::net::TcpListener;

pub static CORS: Lazy<String> = Lazy::new(|| env::var("CORS").expect("CORS policy must be set"));

// This function does the following:
// Loads networking parameters from .env.
// Starts an endless loop that will listen for incoming connections.
// When an incoming connection is found, it spawns a new task to deal with it.
// This task will wait until the entire http request has been collected, then it will send that request off to be processed and responded to in the process_request function.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); //load env vars

    let port = env::var("PORT").expect("DATABASE_URL must be set");
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server listening on port {}", port);

    loop {
        let (mut socket, _) = listener.accept().await?;
        // Spawn a new asynchronous task for each connection.
        tokio::spawn(async move {
            match collect_http_request(&mut socket).await {
                Ok((header, body)) => {
                    handle_http_request(
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
