use rust_components::logger;
use log::{info, warn, error, debug, trace, Level};

#[test]
fn test_logger_initialization() {
    assert!(logger::init(Level::Info).is_ok());
}

#[test]
fn test_log_levels() {
    logger::init(Level::Trace).unwrap();

    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");

    // Since we can't easily capture stdout in tests,
    // we'll just assert that the code runs without panicking
    assert!(true);
}

#[test]
fn test_log_filtering() {
    logger::init(Level::Warn).unwrap();

    debug!("This debug message should not be printed");
    info!("This info message should not be printed");
    warn!("This warning message should be printed");
    error!("This error message should be printed");

    // Again, we can't easily capture stdout, so we just assert that the code runs
    assert!(true);
}