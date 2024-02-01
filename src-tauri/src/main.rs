#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::path::Path;
use std::{env, fs::File};
use zip::write::{FileOptions, ZipWriter};

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run, hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn run(input_paths: Vec<String>, output_path: String) {
    log::info!(
        "Running with files: {:?} and output path: {}",
        input_paths,
        output_path
    );

    let path = Path::new(&output_path);
    let file = File::create(path).unwrap();
    let mut zip = ZipWriter::new(file);

    for file in input_paths {
        let path = Path::new(&file);
        let name = path.file_name().unwrap().to_str().unwrap();
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file(name, options).unwrap();
        let mut f = File::open(path).unwrap();
        io::copy(&mut f, &mut zip).unwrap();
    }

    zip.finish().unwrap();
}
