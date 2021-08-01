#[macro_use]
extern crate log;

mod logger;

// Convenience type for error trait object
type AppError = Box<dyn std::error::Error>;

fn main() -> Result<(), AppError> {
    // Load environment variables from '.env'
    dotenv::dotenv()?;
    // Initialize custom logger
    logger::setup_logger("LOG_LEVEL", "LOG_FILE")?;

    Ok(())
}
