use clap::Parser;
use lazy_static::lazy_static;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Configuration {
    /// Sets the Can interface name
    #[arg(short, long, default_value = None)]
    pub can_interface: Option<String>,

    /// Sets the output log path (must be a folder)
    #[arg(short, long)]
    pub log_path: std::path::PathBuf,

    /// Sets the log file to trace level
    #[arg(short, long, default_value_t = false)]
    pub tracing: bool,

    /// Sets the terminal output to verbose level
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

lazy_static! {
    pub static ref CONFIGURATION: Configuration = Configuration::parse();
}
