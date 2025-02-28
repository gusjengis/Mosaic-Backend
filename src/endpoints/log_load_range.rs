use super::log_common::construct_log_body;
use crate::{db::DB_POOL, log::Log, CORS};
use sqlx::Row;

pub async fn log_load_range(body: String) -> String {
    let mut params = body.split(",");
    let lower: i64 = params.next().unwrap().parse().unwrap();
    let upper: i64 = params.next().unwrap().parse().unwrap();
    match _fetch_log_range(lower, upper).await {
        Ok(logs) => {
            let res_body = construct_log_body(logs);
            let content_length = res_body.len();
            return format!(
                "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: {}\r\nContent-Length: {}\r\n\r\n{}",
                CORS.to_string(), content_length, res_body
            );
        }
        Err(e) => {
            eprintln!("Error in log_load_range(): {}", e);
            return "HTTP/1.1 500 Internal Server Error \r\n\r\n".to_string();
        }
    }
}

async fn _fetch_log_range(
    lower: i64,
    upper: i64,
) -> Result<Vec<Log>, Box<dyn std::error::Error + Send + Sync>> {
    let query = format!(
        "SELECT * FROM logs WHERE timestamp >= {} AND timestamp <= {} ORDER BY timestamp ASC;",
        lower, upper
    );
    let rows = sqlx::query(query.as_str()).fetch_all(&*DB_POOL).await?;

    let logs: Vec<Log> = rows
        .into_iter()
        .map(|row| Log {
            label: row.get("label"),
            timestamp: row.get::<i64, _>("timestamp"),
        })
        .collect();

    Ok(logs)
}
