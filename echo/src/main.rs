use thiserror::Error;
use color_eyre::eyre::Result;
use clap::Parser;

/// Simple echo replacement
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to print to terminal.
    message: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    println!("{}", args.message.unwrap());
    

    Ok (())
}
