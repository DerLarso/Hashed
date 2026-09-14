use chrono::{DateTime, Local, SecondsFormat, Utc};
use std::time::Duration;
#[derive(serde::Serialize)]
pub struct MetaData {
    version: usize,
    files_hashed: usize,
    hash_time: std::time::Duration,
    date_utc: DateTime<Utc>,
    local_time: String,
    os: String,
}

impl MetaData {
    pub fn new(files_hashed: usize, hash_time: Duration, date_utc: DateTime<Utc>) -> MetaData {
        MetaData {
            version: 1,
            files_hashed,
            hash_time,
            date_utc,
            local_time: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            os: std::env::consts::OS.to_string(),
        }
    }

    pub fn get_local_time(&self) -> &str {
        &self.local_time
    }

    pub fn get_current_os(&self) -> &str {
        &self.os
    }
}
