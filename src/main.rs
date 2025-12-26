use arboard::{self, Clipboard};
use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///file input
    file: Option<String>,
    //in future we can make a line arg that allows us to choose a certain line by searching either for the words on the line or the line number itself
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_text = if args.file.is_some() {
        fs::read_to_string(args.file.unwrap()).unwrap()
    } else {
        String::from("")
    };
    let mut clip = arboard::Clipboard::new()?;
    clip.set_text(file_text)?;
    Ok(())
}
