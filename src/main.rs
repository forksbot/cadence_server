#[macro_use]
extern crate log;

mod logger;

// Convenience type for error trait object
type AppError = Box<dyn std::error::Error>;

fn main() -> Result<(), AppError> {
    // Load environment variables from '.env'
    dotenv::dotenv().ok();
    // Initialize custom logger
    logger::init()?;

    let test_msg = "test message";
    trace!("{}", test_msg);
    debug!("{}", test_msg);
    info!("{}", test_msg);
    warn!("{}", test_msg);
    error!("{}", test_msg);

    Ok(())
}
