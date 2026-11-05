use clap::{Parser, Subcommand};
use sha2::{Sha256, Sha512, Digest};
use md5::Md5;
use sha1::Sha1;
use blake2::Blake2b512;
use std::fs::File;
use std::io::{self, Read, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "hash-toolkit", about = "CLI hash toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hash a file
    File {
        path: PathBuf,
        #[arg(short, long, default_value = "sha256")]
        algo: String,
    },
    /// Hash a string
    String {
        text: String,
        #[arg(short, long, default_value = "sha256")]
        algo: String,
    },
    /// Verify file against expected hash
    Verify {
        path: PathBuf,
        expected: String,
        #[arg(short, long, default_value = "sha256")]
        algo: String,
    },
    /// Hash all files in a directory
    Dir {
        path: PathBuf,
        #[arg(short, long, default_value = "sha256")]
        algo: String,
    },
}

fn hash_file(path: &PathBuf, algo: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];

    match algo {
        "md5" => {
            let mut hasher = Md5::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        "blake2b" => {
            let mut hasher = Blake2b512::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput,
                format!("Unknown algo: {}. Use: md5, sha1, sha256, sha512, blake2b", algo))),
    }
}

fn hash_string(text: &str, algo: &str) -> String {
    match algo {
        "md5" => hex::encode(Md5::digest(text.as_bytes())),
        "sha1" => hex::encode(Sha1::digest(text.as_bytes())),
        "sha256" => hex::encode(Sha256::digest(text.as_bytes())),
        "sha512" => hex::encode(Sha512::digest(text.as_bytes())),
        "blake2b" => hex::encode(Blake2b512::digest(text.as_bytes())),
        _ => format!("Unknown algorithm: {}", algo),
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::File { path, algo } => {
            match hash_file(&path, &algo) {
                Ok(h) => println!("{}: {}  {}", algo.to_uppercase(), h, path.display()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::String { text, algo } => {
            let h = hash_string(&text, &algo);
            println!("{}: {}", algo.to_uppercase(), h);
        }
        Commands::Verify { path, expected, algo } => {
            match hash_file(&path, &algo) {
                Ok(h) => {
                    if h.to_lowercase() == expected.to_lowercase() {
                        println!("[MATCH] {}", path.display());
                    } else {
                        println!("[MISMATCH] {}", path.display());
                        println!("  Expected: {}", expected);
                        println!("  Got:      {}", h);
                        std::process::exit(1);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Dir { path, algo } => {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let p = entry.path().to_path_buf();
                    match hash_file(&p, &algo) {
                        Ok(h) => println!("{}  {}", h, p.display()),
                        Err(e) => println!("ERROR: {}  {}", e, p.display()),
                    }
                }
            }
        }
    }
}
