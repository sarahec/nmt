use clap::{Parser, Subcommand};

use clapfig::ConfigArgs;

#[derive(Parser)]
#[command(name = "nmt")]
#[command(version = "0.1.0")]
#[command(about = "Nixpkgs Maintainer's Toolkit", long_about = None)]
struct Cli {
    #[arg(long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage the configuration file (gen, get, set, list).
    Config(ConfigArgs),
}

fn main() {
    let _cli = Cli::parse();
}