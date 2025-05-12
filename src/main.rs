use std::path::PathBuf;
use std::fs;
use clap::Parser;
use colored_hexdump::{hexyl, xxd};

mod binary_template;
use binary_template::binary_template;

#[derive(Parser,Default,Debug)]
//#[command(author, version, about, long_about = None)]
//#[command(propagate_version = true)]
struct Cli {
    /// use a classic xxd style
    #[arg(short)]
    x: bool,

    /// Binary template use to interpret the file
    #[arg(short='t', long)]
    binary_template: Option<PathBuf>,

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

    if let Some(binary_template_path) = cli.binary_template {
        let template = match fs::read_to_string(binary_template_path) {
            Ok(template_content) => template_content,
            Err(e) => {
                eprintln!("Failed to read binary_template: {}", e);
                return;
            }
        };
        
        binary_template(&data, template);
        return;
    }

    let hexdump = match cli.x {
        false => hexyl(&data),
        true  => xxd(&data),
    };

    println!("{hexdump}")
}