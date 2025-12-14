use clap::Parser;
use rand::Rng;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use url::Url;

const STORAGE_FILE: &str = "urls.txt";
const SHORT_ID_LENGTH: usize = 7;
const MAX_RETRIES: u32 = 10;

#[derive(Parser)]
#[command(name = "shortener")]
#[command(about = "A simple URL shortener CLI tool")]
struct Cli {
    /// The long URL to shorten
    url: String,
}

fn main() {
    let cli = Cli::parse();

    // Validate URL
    let url = match Url::parse(&cli.url) {
        Ok(u) => u.to_string(),
        Err(e) => {
            eprintln!("Error: Invalid URL format: {}", e);
            std::process::exit(1);
        }
    };

    // Load existing mappings
    let mappings = match load_mappings(STORAGE_FILE) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: Failed to load mappings: {}", e);
            std::process::exit(1);
        }
    };

    // Check for duplicate
    if let Some(short_id) = find_existing_short_id(&mappings, &url) {
        println!("{}", short_id);
        return;
    }

    // Generate new short ID
    let short_id = match generate_unique_short_id(&mappings, SHORT_ID_LENGTH) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error: Failed to generate unique short ID: {}", e);
            std::process::exit(1);
        }
    };

    // Save new mapping
    if let Err(e) = save_mapping(STORAGE_FILE, &short_id, &url) {
        eprintln!("Error: Failed to save mapping: {}", e);
        std::process::exit(1);
    }

    println!("{}", short_id);
}

fn load_mappings(file_path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let mut mappings = HashMap::new();

    if !Path::new(file_path).exists() {
        return Ok(mappings);
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            mappings.insert(value.to_string(), key.to_string());
        }
    }

    Ok(mappings)
}

fn find_existing_short_id(mappings: &HashMap<String, String>, url: &str) -> Option<String> {
    mappings.get(url).cloned()
}

fn generate_unique_short_id(
    mappings: &HashMap<String, String>,
    length: usize,
) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();

    for _ in 0..MAX_RETRIES {
        let short_id: String = (0..length)
            .map(|_| charset[rng.gen_range(0..charset.len())])
            .collect();

        // Check if this short_id is already used as a key
        if !mappings.values().any(|v| v == &short_id) {
            return Ok(short_id);
        }
    }

    Err(format!(
        "Failed to generate unique short ID after {} attempts",
        MAX_RETRIES
    ))
}

fn save_mapping(file_path: &str, short_id: &str, url: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}={}", short_id, url)?;
    writer.flush()?;

    Ok(())
}

