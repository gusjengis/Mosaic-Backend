use rand::Rng;

use crate::{db::DB_POOL, log::Log};

pub async fn clear_tests() {
    sqlx::query!("DELETE FROM logs WHERE label = 'Test' AND timestamp = 0;",)
        .execute(&*DB_POOL)
        .await;
}

pub async fn delete_log(log: Log) {
    sqlx::query!(
        "DELETE FROM logs WHERE label = $1 AND timestamp = $2;",
        log.label,
        log.timestamp
    )
    .execute(&*DB_POOL)
    .await;
}
