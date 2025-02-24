use std::fmt::Display;

pub struct Log {
    pub label: String,
    pub timestamp: i64,
}

impl Log {
    pub fn from_http_body(body: String) -> Self {
        let mut iter = body.split(",");
        let label = iter.next().unwrap().to_string();
        let timestamp = iter.next().unwrap();
        Self {
            label,
            timestamp: timestamp.parse().unwrap(),
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.label, self.timestamp)
    }
}
