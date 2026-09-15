use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::core::{
    hash_manager::HashAlgorithm::Sha256,
    io::{directory_node::DirectoryNode, meta_data::MetaData},
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
    //own error type is missing!
    pub fn open_file(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> { //doesnt work
        if !Path::new(path).exists() {
            return;
        }
        if !path.ends_with(".hashed") {
            return;
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut content = ZipArchive::new(reader)?;
        let meta: Result<MetaData, serde_json::Error> =
            serde_json::from_reader(content.by_name("meta.json")?);
        let version = match &meta {
            Ok(m) => m.get_version(),
            Err(_m) => 0,
        };
        if version == 1 {
            let mut hash_of_tree = String::from("");
            let result = content
                .by_name("hash.sha256")?
                .read_to_string(&mut hash_of_tree);
            match result {
                Ok(_m) => {
                    let trimmed = hash_of_tree.trim().to_string();
                    let mut s = Vec::new();
                    let r = content.by_name("tree.json")?.read_to_end(&mut s);
                    match r {
                        Ok(_e) => {
                            let hash = Sha256.get_hash_from_bytes(&s);
                            if trimmed == hash {
                                let dic: Result<DirectoryNode, serde_json::Error> =
                                    serde_json::from_slice(&s);
                            } else {
                                Err()
                            }
                        }
                        Err(_e) => (),
                    }
                }
                Err(_e) => (),
            }
        } else {
            Err()
        }
        Ok(String::from(""))
    }
}
