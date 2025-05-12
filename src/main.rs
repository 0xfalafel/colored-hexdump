use std::path::PathBuf;
use std::fs;
use clap::Parser;
use colored_hexdump::{hexyl, xxd};

#[derive(Parser,Default,Debug)]
//#[command(author, version, about, long_about = None)]
//#[command(propagate_version = true)]
struct Cli {
    /// use a classic xxd style
    #[arg(short)]
    x: bool,

    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = match fs::read(cli.file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let hexdump = match cli.x {
        false => hexyl(&data),
        true  => xxd(&data),
    };

    println!("{hexdump}")
}