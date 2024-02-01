#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::io::{self, Write};
use std::path::Path;
use std::{env, fs::File};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

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

#[derive(Serialize, Deserialize)]
struct FormData {
    name: String,
    description: String,
}

#[tauri::command]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn run(input_paths: Vec<String>, output_path: String, form_data: FormData) {
    log::info!(
        "Running with files: {:?} and output path: {}",
        input_paths,
        output_path
    );

    let path = Path::new(&output_path);
    let file = File::create(path).unwrap();
    let mut zip = ZipWriter::new(file);

    let form_data_json = to_string(&form_data).unwrap();

    zip.start_file(
        "form_data.json",
        FileOptions::default().compression_method(CompressionMethod::Stored),
    )
    .unwrap();
    zip.write_all(form_data_json.as_bytes()).unwrap();

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
