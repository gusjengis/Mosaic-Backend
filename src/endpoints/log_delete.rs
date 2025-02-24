use crate::{db::DB_POOL, log::Log};

pub async fn log_delete(body: String) -> String {
    let log = Log::from_http_body(body);
    if let Err(e) = sqlx::query!(
        "DELETE FROM logs WHERE label = $1 AND timestamp = $2;",
        log.label,
        log.timestamp
    )
    .execute(&*DB_POOL)
    .await
    {
        eprintln!("Error in log_delete(): {}", e);
    }

    return "HTTP/1.1 200 OK\r\n\r\n".to_string();
}
