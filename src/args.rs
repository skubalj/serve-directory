//! Command line argument parsing

use clap::Parser;
use std::path::PathBuf;

/// Serve files in the specified directory and subdirectories
#[derive(Debug, Parser)]
#[command(version)]
pub struct Arguments {
    /// The root directory that should be served
    #[arg(default_value = ".")]
    pub folder: PathBuf,

    /// The port that the server should bind to
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Deploy on localhost, rather than binding to the external interface
    #[arg(short, long)]
    pub localhost: bool,

    /// Log additional data to the console
    #[arg(short, long)]
    pub verbose: bool,
}
