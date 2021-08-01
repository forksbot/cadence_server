use std::env;
use std::fs::File;

use fern::{Dispatch, InitError};
use log::LevelFilter;

const LOG_LEVEL_DEFAULT: &str = "DEBUG";
const LOG_FILE_DEFAULT: &str = "logs/server-dev.log";

/// Initialize the global `Logger` instance using environment variables
pub fn setup_logger(level: &str, file_out: &str) -> Result<(), InitError> {
    // TODO: Cleanup this logic; increase readability
    let log_level = env::var(level)
        .unwrap_or_else(|_| LOG_LEVEL_DEFAULT.into())
        .parse()
        .unwrap_or(LevelFilter::Info);

    let log_file = env::var(file_out).unwrap_or_else(|_| LOG_FILE_DEFAULT.into());
    let file_logger = File::create(log_file)?;

    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr())
        .chain(file_logger);

    // Build this logger and instantiate it as the global `log` logger
    dispatch.apply()?;

    Ok(())
}
