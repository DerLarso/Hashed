use crate::core::hash_manager::{HashAlgorithm, HashManager};
use crate::core::structs::file_info::FileInfo;
use std::io::{self, Error};
use std::path::Path;
use std::time::{Duration, SystemTime};

pub struct ScanManager {
    file_list: Vec<FileInfo>,
    hasher: HashManager,
    start_path: String,
    hash_time: Duration,
}

impl ScanManager {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        ScanManager {
            file_list: Vec::new(),
            hasher: HashManager::new(algorithm),
            start_path: String::new(),
            hash_time: Duration::ZERO,
        }
    }

    pub fn start_hash(&mut self, path: String) -> io::Result<()> {
        let time = SystemTime::now();
        self.start_path = path.clone();
        let path_check = Path::new(&path);

        if !path_check.exists() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "This Path is invalid!",
            ));
            //TODO: Replace with own error type
        }

        self.recursive_read(path_check);

        self.hash_time = time.elapsed().unwrap_or_default();
        Ok(())
    }

    fn recursive_read(&mut self, path_buf: &Path) {
        let path = path_buf.as_os_str().to_str().unwrap().to_string();
        if cfg!(target_os = "linux")
            && (path.starts_with("/proc/")
                || path.starts_with("/sys/")
                || path.starts_with("/dev/")
                || path.starts_with("/run/"))
        {
            return;
        }


        if cfg!(target_os = "windows")
            && (path.ends_with("pagefile.sys") || path.ends_with("hiberfil.sys"))
        {
            return;
        }
        let check = Path::new(&path);
        let Ok(metadata) = check.symlink_metadata() else {
            return;
        };

        if metadata.is_symlink() {
            return;
        }

        if metadata.is_file() {
            let result = self.compute_hash(&path);

            self.file_list.push(FileInfo::new(path, result));
        } else if metadata.is_dir()
            && let Ok(entries) = check.read_dir()
        {
            for entry in entries.flatten() {
                let entry_path = &entry.path();

                self.recursive_read(entry_path);
            }
        }
    }

    pub fn get_list(&self) -> &Vec<FileInfo> {
        &self.file_list
    }

    fn compute_hash(&mut self, path: &str) -> Result<String, Error> {
        self.hasher.get_hash_code(path)
    }

    pub fn get_hash_manager(&self) -> &HashManager {
        &self.hasher
    }

    pub fn get_path(&self) -> &str {
        &self.start_path
    }

    pub fn get_hash_time(&self) -> &Duration {
        &self.hash_time
    }
}
