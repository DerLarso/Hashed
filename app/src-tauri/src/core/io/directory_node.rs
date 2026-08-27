use std::collections::HashMap;

use crate::core::structs::file_info::FileInfo;
use std::io::{BufWriter, Write};
use std::fs::{File};
#[derive(serde::Serialize)]
pub struct DirectoryNode {
    pub name: String,
    pub files: HashMap<String, Result<String, String>>,
    pub sub_directories: HashMap<String, DirectoryNode>,
}

impl DirectoryNode {
    pub fn new(name: String) -> Self {
        DirectoryNode {
            name,
            files: HashMap::new(),
            sub_directories: HashMap::new(),
        }
    }

    pub fn insert_path(&mut self, path_parts: &[&str], result: Result<String, String>) {
        if path_parts.is_empty() {
            return;
        }

        if path_parts.len() == 1 {
            let file_name = path_parts[0].to_string();

            self.files.insert(file_name, result);
        } else {
            let folder_name = path_parts[0].to_string();

            let next_node = self
                .sub_directories
                .entry(folder_name.clone())
                .or_insert_with(|| DirectoryNode::new(folder_name));

            next_node.insert_path(&path_parts[1..], result);
        }
    }

    pub fn build_tree(start_path: &str, file_list: &Vec<FileInfo>) -> DirectoryNode {
        let mut start = DirectoryNode::new(start_path.to_string());

        for file in file_list {
            let path = file.get_path();

            let path_parts: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();

            let result = match file.get_result() {
                Ok(a) => Ok(a.clone()),
                Err(e) => Err(e.to_string()),
            };

            start.insert_path(&path_parts, result);
        }

        start
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }
    
}
