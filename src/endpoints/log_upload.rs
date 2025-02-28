use crate::{db::DB_POOL, log::Log, CORS};

pub async fn log_upload(body: String) -> String {
    let log = Log::from_http_body(body);

    if let Err(e) = sqlx::query!(
        "INSERT INTO logs (label, timestamp) VALUES ($1, $2)",
        log.label,
        log.timestamp,
    )
    .execute(&*DB_POOL)
    .await
    {
        eprintln!("Error in log_upload(): {}", e);
    }

    format!(
        "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: {}\r\n\r\n",
        CORS.to_string()
    )
}
