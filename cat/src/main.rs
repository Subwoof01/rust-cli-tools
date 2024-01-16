use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Simple program to print a file's contents to the terminal.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to print.
    file: String, 
}

fn main() -> std::io::Result<()>{
    // Parse the input arguments. (File path).
    let args = Args::parse();

    // Open the file stream.
    let f = File::open(args.file)?;
    let reader = BufReader::new(f);

    // Read each line
    for line in reader.lines().flatten() {
        println!("{}", line);
    }

    Ok(())
}
