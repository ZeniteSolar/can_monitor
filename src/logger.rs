use crate::cli;

use anyhow::Result;
use tracing::{metadata::LevelFilter, *};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

// Start logger, should be done inside main
pub fn init() -> Result<()> {
    // Redirect all logs from libs using "Log"
    LogTracer::init_with_filter(tracing::log::LevelFilter::Trace).expect("Failed to set logger");

    // Configure the console log
    let console_env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        if cli::CONFIGURATION.verbose {
            EnvFilter::new(LevelFilter::DEBUG.to_string())
        } else {
            EnvFilter::new(LevelFilter::INFO.to_string())
        }
    });
    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(fmt::format::FmtSpan::NONE)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_filter(console_env_filter);

    // Configure the file log
    let file_env_filter = if cli::CONFIGURATION.tracing {
        EnvFilter::new(LevelFilter::TRACE.to_string())
    } else {
        EnvFilter::new(LevelFilter::DEBUG.to_string())
    };
    let dir = &cli::CONFIGURATION.log_path;
    let file_appender = tracing_appender::rolling::hourly(dir, "can_monitor.log");
    let file_layer = fmt::Layer::new()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(fmt::format::FmtSpan::NONE)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_filter(file_env_filter);

    // Configure the default subscriber
    let subscriber = tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");

    info!(
        "Starting at {}",
        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
    );

    Ok(())
}
