// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizerTech
 */

use clap::Parser;
use artificialintelligenceoptimizertech::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizerTech - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
