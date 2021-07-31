use std::env;
use std::fs::File;

use fern::{Dispatch, InitError};
use log::LevelFilter;

/// Initialize the global `Logger` instance
pub fn init() -> Result<(), InitError> {
    // Get the default log level from the environment; else set to `DEBUG`
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "DEBUG".into());
    let log_level = log_level
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    let mut dispatch = Dispatch::new()
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
        .chain(std::io::stderr());

    // Enable logging to file if a log file is provided
    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = File::create(log_file)?;
        dispatch = dispatch.chain(log_file);
    }

    // Build this logger and instantiate it as the global `log` logger
    dispatch.apply()?;

    Ok(())
}
