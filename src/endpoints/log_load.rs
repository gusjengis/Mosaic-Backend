use crate::{db::DB_POOL, log::Log, CORS};
use sqlx::Row;

use super::log_common::construct_log_body;

pub async fn log_load() -> String {
    match _fetch_all_logs().await {
        Ok(logs) => {
            let res_body = construct_log_body(logs);
            let content_length = res_body.len();
            return format!(
                "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: {}\r\nContent-Length: {}\r\n\r\n{}",
                CORS.to_string(), content_length, res_body
            );
        }
        Err(e) => {
            eprintln!("Error in log_load(): {}", e);
            return "HTTP/1.1 500 Internal Server Error \r\n\r\n".to_string();
        }
    }
}

async fn _fetch_all_logs() -> Result<Vec<Log>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query("SELECT * FROM logs ORDER BY timestamp ASC;")
        .fetch_all(&*DB_POOL)
        .await?;

    let logs: Vec<Log> = rows
        .into_iter()
        .map(|row| Log {
            label: row.get("label"),
            timestamp: row.get::<i64, _>("timestamp"),
        })
        .collect();

    Ok(logs)
}
