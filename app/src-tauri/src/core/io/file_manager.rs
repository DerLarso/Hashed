use std::{
    fs::File,
    io::{BufWriter, Write},
};

use zip::{ZipWriter, write::SimpleFileOptions};

use crate::core::{
    hash_manager::HashAlgorithm::Sha256,
    io::{directory_node::DirectoryNode, meta_data::MetaData},
};

pub struct FileManager {}

impl FileManager {
    pub fn save_file(
        folder: &str,
        json: &DirectoryNode,
        meta: &MetaData,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
}
