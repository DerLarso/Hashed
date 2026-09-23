use crate::core::io::{directory_node::DirectoryNode, meta_data::MetaData};


pub struct HashedFileStruct {
    meta: MetaData,
    tree: DirectoryNode,
    hash: String,
}

impl HashedFileStruct {
    pub fn new(meta: MetaData, tree: DirectoryNode, hash: String) -> HashedFileStruct{
        HashedFileStruct { meta, tree, hash }
    }
    pub fn get_meta(&self) -> &MetaData {
        &self.meta
    }
    pub fn get_tree(&self) -> &DirectoryNode {
        &self.tree
    }
    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}