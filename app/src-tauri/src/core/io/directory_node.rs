use std::collections::BTreeMap;

use crate::core::structs::file_info::FileInfo;
#[derive(serde::Serialize)]
pub struct DirectoryNode {
    pub name: String,
    pub files: BTreeMap<String, Result<String, String>>,
    pub sub_directories: BTreeMap<String, DirectoryNode>,
}

impl DirectoryNode {
    pub fn new(name: String) -> Self {
        DirectoryNode {
            name,
            files: BTreeMap::new(),
            sub_directories: BTreeMap::new(),
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

    pub fn build_tree(start_path: &str, file_list: &[FileInfo]) -> DirectoryNode {
        let mut start = DirectoryNode::new(start_path.to_string());

        for file in file_list {
            let path = std::path::Path::new(file.get_path());

            let path_parts: Vec<&str> = path
                .components()
                .filter_map(|a| a.as_os_str().to_str())
                .filter(|s| *s != "/" && !s.contains(':'))
                .collect();

            let result = match file.get_result() {
                Ok(a) => Ok(a.clone()),
                Err(e) => Err(e.to_string()),
            };

            start.insert_path(&path_parts, result);
        }

        start
    }
}
