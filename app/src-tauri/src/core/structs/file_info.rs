pub struct FileInfo {
    path: String,
    result: Result<String, std::io::Error>,
}

impl FileInfo {
    pub fn new(path: String, result: std::io::Result<String>) -> Self {
        FileInfo { path, result }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_result(&self) -> &Result<String, std::io::Error> {
        &self.result
    }
}
