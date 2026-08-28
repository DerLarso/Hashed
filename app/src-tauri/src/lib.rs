mod core;
use std::thread;

use crate::core::{
    hash_manager::HashAlgorithm,
    io::{directory_node::DirectoryNode, file_manager::FileManager, meta_data::MetaData},
    scan_manager::ScanManager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // SAFETY: this program is single-threaded up to this point
            unsafe {
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            }
        }
    }

    tauri::Builder::default()
        .setup(|_app| {
            thread::spawn(move || {
                cli();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error");
}

//temp
fn cli() {
    let mut scan = ScanManager::new(HashAlgorithm::Blake3);

    let res = scan.start_hash(String::from("/home/lars/Bilder"));

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
    let meta = MetaData::new(
        *s.get_hash_manager().get_files_counted(),
        *s.get_hash_time(),
    );
    let output = FileManager::save_file("/home/lars/Downloads/test.hashed", &test, &meta);
    match output {
        Ok(()) => (),
        Err(_e) => println!("Error"),
    }
}
