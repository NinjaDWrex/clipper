use clap::*;
use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// file input
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file_text = if let Some(path) = args.file {
        fs::read_to_string(path).expect("error reading file")
    } else {
        String::new()
    };

    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to start wl-copy");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(file_text.as_bytes())
        .expect("failed to write to wl-copy");

    child.wait().expect("wl-copy failed");
}
