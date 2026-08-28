use std::time::{Duration};

#[derive(serde::Serialize)]
pub struct MetaData {
    version: usize,
    files_hashed: usize,
    hash_time: std::time::Duration,
}

impl MetaData {
    pub fn new(files_hashed: usize, hash_time: Duration) -> MetaData {
        MetaData {
            version: 1,
            files_hashed,
            hash_time,
        }
    }
}
