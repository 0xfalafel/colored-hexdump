use std::path::PathBuf;
use std::fs;
use clap::Parser;
use colored_hexdump::{hexyl, xxd_braille, BrailleMode};

#[derive(Parser,Default,Debug)]
//#[command(author, version, about, long_about = None)]
//#[command(propagate_version = true)]
struct Cli {
    /// use a classic xxd style
    #[arg(short)]
    x: bool,

    /// Use full braille for the ascii panel.
    #[arg(short='b', long)]
    braille: bool,

    /// No braille, use 'x' for non-ascii chars.
    #[arg(short='B', long)]
    no_braille: bool,

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

    let braille = match (cli.no_braille, cli.braille) {
        (true, _)  => BrailleMode::None,
        (_, true)  => BrailleMode::All,
        (_, false) => BrailleMode::Mixed,
    };

    let hexdump = match cli.x {
        false => hexyl(&data, braille),
        true  => xxd_braille(&data, braille),
    };

    println!("{hexdump}")
}