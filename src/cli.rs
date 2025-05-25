use clap::Parser;
use lazy_static::lazy_static;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Configuration {
    /// Sets the Can interface name
    #[arg(short, long, default_value = None)]
    pub can_interface: Option<String>,

    /// Sets the output log path (must be a folder)
    #[arg(short, long, default_value = ".")]
    pub log_path: std::path::PathBuf,

    /// Sets the log file to trace level
    #[arg(short, long, default_value_t = false)]
    pub tracing: bool,

    /// Sets the terminal output to verbose level
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Sets the interval time to send data, in milliseconds
    #[arg(short, long, default_value_t = 1)]
    pub period: u64,

    /// Sets the server address
    #[arg(short, long, default_value_t = String::from("0.0.0.0:3000"))]
    pub address: String,

    /// Skips CAN interface initialization (for local dev)
    #[arg(long, default_value_t = false)]
    pub no_can: bool,

    /// Skips CAN interface initialization (for local dev)
    #[arg(long, default_value_t = false)]
    pub no_log: bool,
}

lazy_static! {
    pub static ref CONFIGURATION: Configuration = Configuration::parse();
}