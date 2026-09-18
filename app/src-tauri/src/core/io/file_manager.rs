use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::core::{
    hash_manager::HashAlgorithm::Sha256, io::{directory_node::DirectoryNode, hashed_error::HashedError, hashed_file_struct::HashedFileStruct, meta_data::MetaData},
};

pub struct FileManager {}

impl FileManager {
    pub fn save_file(
        mut folder: String,
        json: &DirectoryNode,
        meta: &MetaData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        folder = folder + meta.get_current_os() + "-" + meta.get_local_time() + ".hashed";

        let path = File::create(folder)?;
        let writer = BufWriter::new(path);
        let mut zip = ZipWriter::new(writer);

        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("meta.json", options)?;
        serde_json::to_writer(&mut zip, meta)?;

        let tree = serde_json::to_vec(json)?;
        let hash = Sha256.get_hash_from_bytes(&tree);

        zip.start_file("tree.json", options)?;
        zip.write_all(&tree)?;

        zip.start_file("hash.sha256", options)?;
        zip.write_all(hash.as_bytes())?;

        zip.finish()?;
        Ok(())
    }

    pub fn open_file(&self, path: &str) -> Result<HashedFileStruct, HashedError> {
        if !Path::new(path).exists() {
            return Err(HashedError::FileNotFound(path.to_string()));
        }
        if !path.ends_with(".hashed") {
            return Err(HashedError::InvalidEnding(path.to_string()));
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut content = ZipArchive::new(reader)?;
        let meta: MetaData = serde_json::from_reader(content.by_name("meta.json")?)?;
        let version = meta.get_version();
        if version == 1 {
            let mut hash_of_tree = String::from("");
            content
                .by_name("hash.sha256")?
                .read_to_string(&mut hash_of_tree)?;
            let trimmed = hash_of_tree.trim().to_string();
            let mut s = Vec::new();
            content.by_name("tree.json")?.read_to_end(&mut s)?;
            let hash = Sha256.get_hash_from_bytes(&s);
            if trimmed == hash {
                let dic: DirectoryNode = serde_json::from_slice(&s)?;
                Ok(HashedFileStruct::new(meta,dic,hash))
            } else {
                Err(HashedError::HashMissmatch {
                    expected: trimmed,
                    actual: hash,
                })
            }
        } else {
            Err(HashedError::UnsupportedVersion(version))
        }
    }
}
