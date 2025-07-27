// src/main.rs
/*
 * Main executable for HyperBrand
 */

use clap::Parser;
use hyperbrand::{Result, run};

#[derive(Parser)]
#[command(version, about = "HyperBrand - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
