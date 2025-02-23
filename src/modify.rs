use rand::Rng;

use crate::db::DB_POOL;

pub async fn clear_tests() {
    sqlx::query!("DELETE FROM logs WHERE label = 'Test' AND timestamp = 0;",)
        .execute(&*DB_POOL)
        .await;
}
