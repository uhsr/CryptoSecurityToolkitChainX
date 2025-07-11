// src/main.rs
/*
 * Main executable for CryptoSecurityToolkitChainX
 */

use clap::Parser;
use cryptosecuritytoolkitchainx::{Result, run};

#[derive(Parser)]
#[command(version, about = "CryptoSecurityToolkitChainX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
