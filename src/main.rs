use std::io::prelude::*;
use clap::Parser;
use anyhow:: {Context, Result};
use std::{fs::File, io::BufReader};
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf
}
fn main(){
    // Parsing the command from the CLI
    let args = Cli::parse();
    // Passing the pattern to look for and the file to look into the read function
    let result = read_through_buffer(&args.pattern, &args.path);
    if let Err(e) = result {
        eprintln!("{}", e);
    }
}


// Implementing BufReader to improve performcance and use less memory while reading large files
fn read_through_buffer(pattern: &str, path: &std::path::PathBuf) -> Result<()>{
    // A handle to the standard output
    let stdout = io::stdout();
    // Wrapping a buffer around the handle and locking it
    let mut writer = io::BufWriter::new(stdout.lock());
    // Opening the file to read and adding context to the Error (if any) using anyhow::Context
    let f = File::open(path)
    .with_context(|| format!("Error reading file '{}'", path.display()))?;
    // Wrapping a buffer to read the file
    let reader = BufReader::new(f);
    // Reading through the file line by line 
    let mut success = false;
    for line_result in reader.lines() {
        let line = line_result.with_context(|| format!("Failed to read line"))?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
            success = true;
        }
    }
    if !success {
        println!("Pattern not found!");
    }
    // Flushing the buffer for any remaining line in the memory
    writer.flush()?;
    Ok(())
}
