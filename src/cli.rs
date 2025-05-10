use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
/// A TCP/TLS loadbalancer
pub struct Cli {
    /// Path to the config file
    #[arg(short, long, value_name = "CONFIG_FILE")]
    pub config_file: PathBuf,
}
