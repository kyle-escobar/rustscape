#![feature(windows_process_extensions_async_pipes)]

use std::{env, io, thread};
use std::fs::read_dir;
use std::io::stdout;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{exit, Command, Stdio};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

macro_rules! e {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

fn main() {
    rerun_directory("../core/");

    compile_core();
}

fn rerun_directory<T: AsRef<Path> + ?Sized>(dir: &T) {
    println!("cargo:rerun-if-changed={}", dir.as_ref().to_str().unwrap());
    for entry in read_dir(dir).unwrap() {
        let entry = entry.expect("Could not read file in dir.");
        let path = entry.path().to_path_buf();
        if !path.is_dir() {
            continue;
        }
        rerun_directory(&path);
    }
}

fn compile_core() {
    p!("[+] Compiling RustScape Core DLL...");

    let path = Path::new("../core/").to_path_buf();
    let cargo_path = &mut path.clone();
    cargo_path.push("Cargo.toml");

    env::set_current_dir(path).unwrap();
    thread::spawn(move || {
        let output = Command::new("cargo")
            .env("CFLAGS", "-lrt")
            .env("LDFLAGS", "-lrt")
            .env("RUSTFLAGS", "-C target-feature=+crt-static")
            .arg("build")
            //.arg("--release")
            .arg("-vv")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .async_pipes(true)
            .output()
            .unwrap();
        e!("{}", String::from_utf8_lossy(&output.stderr));
        p!("{}", String::from_utf8_lossy(&output.stdout));
    });
}
