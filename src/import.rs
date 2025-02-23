use sqlx::Row;
use std::error::Error; // Import the Row trait for the .get() method

use crate::{db::DB_POOL, log::Log};

pub async fn fetch_data(data_type: &str, body: String) -> String {
    match data_type {
        "logLoad" => match fetch_all_logs().await {
            Ok(logs) => {
                return construct_log_body(logs);
            }
            Err(e) => {
                eprintln!("Error in logDownload: {}", e);
                return "".to_string();
            }
        },
        "logLoadRange" => {
            let mut params = body.split(",");
            let lower: i64 = params.next().unwrap().parse().unwrap();
            let upper: i64 = params.next().unwrap().parse().unwrap();
            match fetch_log_range(lower, upper).await {
                Ok(logs) => {
                    return construct_log_body(logs);
                }
                Err(e) => {
                    eprintln!("Error in logDownload: {}", e);
                    return "".to_string();
                }
            }
        }
        "testDBConnection" => {
            if let Err(e) = fetch_test().await {
                eprintln!("Error in fetch_test: {}", e);
            }
        }
        _ => {}
    }

    "".to_string()
}

async fn fetch_log_range(
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

fn construct_log_body(logs: Vec<Log>) -> String {
    let mut res = "".to_string();
    for log in logs {
        res.push_str(format!("{}\n", log.to_string()).as_str());
    }
    res = res.trim_end().to_string();

    res
}

pub async fn fetch_all_logs() -> Result<Vec<Log>, Box<dyn std::error::Error + Send + Sync>> {
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

pub async fn fetch_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&*DB_POOL)
        .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    Ok(())
}
