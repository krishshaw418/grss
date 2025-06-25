// use std::io::prelude::*;
use clap::Parser;
use anyhow:: {Context, Result};
// use std::{fs::File, io::BufReader};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf
}
fn main() -> Result<()>{
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).with_context(|| format!("Could not read file {}", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    };
    Ok(())
}


// Implementing BufReader to improve performcance and use less memory while reading large files
// fn read_through_buffer(pattern: &str, path: &std::path::PathBuf) -> std::io::Result<()>{
//     let f = File::open(path)?;
//     let reader = BufReader::new(f);
//     for line_result in reader.lines() {
//         match line_result {
//             Ok(line) => {
//                 if line.contains(pattern) {
//                     println!("{}", line);
//                 }
//             },
//             Err(_) => println!("Error reading lines!"),
//         }
//     }
//     Ok(())
// }