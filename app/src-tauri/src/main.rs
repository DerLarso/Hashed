// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
use crate::core::{
    hash_manager::HashAlgorithm, io::directory_node::DirectoryNode, scan_manager::ScanManager,
};
fn main() {
    //hashed_lib::run();
    let mut scan = ScanManager::new(HashAlgorithm::Blake3);

    let res = scan.start_hash(String::from("/home/lars/Games"));

    match res {
        Ok(()) => handle_success(&mut scan),
        Err(e) => println!("{}", e),
    }
    println!("{}", scan.get_hash_manager().get_files_counted());
}

fn handle_success(s: &mut ScanManager) {
    let finished_list = s.get_list();

    for i in finished_list.iter() {
        let result = i.get_result();
        match result {
            Ok(s) => println!("Path: {}; Result: {}", i.get_path(), s),
            Err(s) => println!("Path: {}; Result: {}", i.get_path(), s),
        }
    }

    let test = DirectoryNode::build_tree(s.get_path(), finished_list);
    let json = test.to_json();
    match json {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
    let output = test.save_to_file("/home/lars/Downloads/test.json");
    match output {
        Ok(()) => (),
        Err(_e) => println!("Error"),
    }
}
