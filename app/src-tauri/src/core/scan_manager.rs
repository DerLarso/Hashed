use std::io::{self, Error};
use std::path::Path;

use crate::core::hash_manager::{HashAlgorithm, HashManager};
use crate::core::structs::file_info::FileInfo;

pub struct ScanManager {
    file_list: Vec<FileInfo>,
    hasher: HashManager,
}

impl ScanManager {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        ScanManager {
            file_list: Vec::new(),
            hasher: HashManager::new(algorithm),
        }
    }

    pub fn start_hash(&mut self, path: String) -> io::Result<()> {
        let path_check = Path::new(&path);

        if !path_check.exists() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "This Path is invalid!",
            ));
            //TODO: Replace with own error type
        }

        self.recursive_read(path);

        Ok(())
    }

    fn recursive_read(&mut self, path: String) {
        if cfg!(target_os = "linux")
            && (path == "/proc" || path == "/sys" || path == "/dev" || path == "/run")
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
                let entry_path = entry.path().display().to_string();

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
}
