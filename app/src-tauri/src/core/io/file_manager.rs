use std::{fs::File, io::BufWriter};

use zip::{ZipWriter, write::SimpleFileOptions};

use crate::core::io::{directory_node::DirectoryNode, meta_data::MetaData};

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

        zip.start_file("tree.json", options)?;
        serde_json::to_writer(&mut zip, json)?;

        zip.finish()?;
        Ok(())
    }
}
