use std::io::prelude::*;
use clap::Parser;
use anyhow:: {Context, Result};
use std::{fs::File, io::BufReader};
use std::io::{self, Write};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

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

    // Adding a spinner to show work in progress
    let bar = ProgressBar::new_spinner();
    bar.set_style(ProgressStyle::with_template("{spinner} Scanning...").unwrap());
    bar.enable_steady_tick(Duration::from_millis(100)); // Makes the spinner rotate

    // A handle to the standard output
    let stdout = io::stdout();

    // Wrapping a buffer around the handle and locking it
    let mut writer = io::BufWriter::new(stdout.lock());

    // Opening the file to read and adding context to the Error (if any) using anyhow::Context
    let f = File::open(path)
    .with_context(|| format!("Error reading file '{}'", path.display()))?;

    // Wrapping a buffer to read the file
    let reader = BufReader::new(f);

    // To check for existence of the input pattern
    let mut success = false;
    let mut line_count = 0;

    // Reading the file line by line and inspecting each line for the given pattern
    for line_result in reader.lines() {
        let line = line_result.with_context(|| format!("Failed to read line {}", line_count + 1))?;
        line_count += 1;
        // To simulate heavy work(when large file is read) so that the spinner is visible
        std::thread::sleep(std::time::Duration::from_millis(5));
        bar.tick(); 
        if line.contains(pattern) {
            writeln!(writer, "{} at line number: {}", line, line_count)?;
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
