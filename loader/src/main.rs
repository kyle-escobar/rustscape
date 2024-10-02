mod injector;

use log::LevelFilter;
use rust_embed::Embed;
use simple_logger::SimpleLogger;
use std::fs;
use std::path::PathBuf;

#[derive(Embed)]
#[folder = "../target/release/"]
#[include = "rustscape.dll"]
struct EmbedData;

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    log::info!("Starting RustScape Loader...");

    log::info!("Extracting packed DLL data for injection.");
    let file = EmbedData::get("rustscape.dll").unwrap();
    let file_bytes = file.data.to_vec();

    let tmp = &mut mktemp::Temp::new_dir().unwrap();

    let mut dll_file_path = tmp.to_path_buf();
    dll_file_path = PathBuf::from(format!("{}\\rustscape.dll", dll_file_path.to_str().unwrap()));

    let dll_file_path = dll_file_path.as_path();
    fs::write(dll_file_path, &file_bytes).unwrap();

    log::info!("Successfully extracted '{}' from current executable.", dll_file_path.file_name().unwrap().to_str().unwrap());

    /*
     * Inject the dll into the first running process with name 'osclient.exe' that doesnt have a
     * loaded module named 'rustscape.dll' already loaded.
     */
    injector::inject_dll("osclient.exe", dll_file_path).unwrap();
    log::info!("RustScape loader completed successfully. Exiting...");
}
