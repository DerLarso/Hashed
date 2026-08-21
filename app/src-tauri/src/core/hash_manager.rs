use blake3;
use hex;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, Read};

pub struct HashManager {
    files_counted: usize,
    algorithm: HashAlgorithm,
}
pub enum HashAlgorithm {
    Sha256,
    Blake3,
}

impl HashManager {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        HashManager {
            files_counted: 0,
            algorithm,
        }
    }

    pub fn get_hash_code(&mut self, path: &str) -> io::Result<String> {
        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        let mut buffer: [u8; 65536] = [0u8; 64 * 1024];

        let hex_string = match self.algorithm {
            HashAlgorithm::Blake3 => {
                let mut hasher = blake3::Hasher::new();

                loop {
                    let read_bytes = reader.read(&mut buffer)?;

                    if read_bytes == 0 {
                        break;
                    }

                    let byte_part = &buffer[..read_bytes];

                    hasher.update(byte_part);
                }

                let final_hash = hasher.finalize();

                final_hash.to_string()
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();

                loop {
                    let read_bytes = reader.read(&mut buffer)?;

                    if read_bytes == 0 {
                        break;
                    }

                    let byte_part = &buffer[..read_bytes];

                    hasher.update(byte_part);
                }

                let final_hash = hasher.finalize();

                hex::encode(final_hash)
            }
        };

        self.files_counted += 1;

        Ok(hex_string)
    }

    pub fn get_files_counted(&self) -> &usize {
        &self.files_counted
    }
}
