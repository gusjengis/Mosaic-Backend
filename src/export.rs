use crate::db::*;
use crate::log::Log;

pub fn forward_data(data_type: &str, body: String) {
    match data_type {
        "logUpload" => {
            let log = Log::from_http_body(body);
            println!("logUpload: {}", log.to_string());
            tokio::spawn(upload_log(log));
        }
        _ => {}
    }
}

pub async fn upload_log(log: Log) {
    sqlx::query!(
        "INSERT INTO logs (label, timestamp) VALUES ($1, $2)",
        log.label,
        log.timestamp as i64,
    )
    .execute(&*DB_POOL)
    .await;
}
