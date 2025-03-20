use log::{LevelFilter, info, warn};
use simplelog::{Config, TermLogger, TerminalMode};

pub fn setup_logger(log_level: &str) {
    let level = match log_level {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    TermLogger::init(level, Config::default(), TerminalMode::Mixed)
        .expect("Failed to initialize logger");

    info!("Logger initialized with level: {}", log_level);
}

