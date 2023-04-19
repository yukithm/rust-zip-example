use core::panic;
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, Write},
    path::Path,
};

use clap::{Parser, Subcommand};
use zip::{result::ZipError, ZipArchive};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(visible_alias = "ls", about = "list entries")]
    List {
        #[arg(help = "zip file")]
        zip_file: String,
    },
    #[command(visible_alias = "x", about = "extract entry")]
    Extract {
        #[arg(help = "zip file")]
        zip_file: String,
        #[arg(help = "entry name")]
        entry_name: String,
    },
    #[command(about = "print entry content")]
    Cat {
        #[arg(help = "zip file")]
        zip_file: String,
        #[arg(help = "entry name")]
        entry_name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { zip_file } => {
            list_entries(&zip_file);
        }
        Commands::Extract {
            zip_file,
            entry_name,
        } => {
            extract_entry(&zip_file, &entry_name);
        }
        Commands::Cat {
            zip_file,
            entry_name,
        } => {
            print_entry(&zip_file, &entry_name);
        }
    }
}

fn list_entries<P: AsRef<Path>>(zip_file: P) {
    let archive = open_zip(zip_file).unwrap();
    for entry in archive.file_names() {
        println!("{}", entry);
    }
}

fn extract_entry<P: AsRef<Path>>(zip_file: P, entry_name: &str) {
    println!(
        "extract: zip={}, entry={}",
        zip_file.as_ref().to_string_lossy(),
        entry_name
    );
}

fn print_entry<P: AsRef<Path>>(zip_file: P, entry_name: &str) {
    let mut archive = open_zip(zip_file.as_ref()).unwrap();
    let mut entry = match archive.by_name(entry_name) {
        Ok(entry) => entry,
        Err(ZipError::FileNotFound) => {
            panic!(
                "{} not found in {}.",
                entry_name,
                zip_file.as_ref().to_string_lossy()
            )
        }
        Err(error) => {
            panic!("error: {:?}", error)
        }
    };

    if entry.is_dir() {
        panic!("{entry_name} is a directory.")
    }

    let mut buf = [0u8; 1024];
    loop {
        match entry.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => io::stdout().write_all(&buf[..n]).unwrap(),
            Err(e) => panic!("error: {}", e),
        }
    }
}

fn open_zip<P: AsRef<Path>>(
    zip_file: P,
) -> Result<ZipArchive<impl Read + Seek>, Box<dyn std::error::Error>> {
    let file = File::open(zip_file)?;
    let archive = ZipArchive::new(BufReader::new(file))?;
    Ok(archive)
}
