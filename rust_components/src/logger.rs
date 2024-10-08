use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use chrono::Local;
use std::sync::Once;

pub struct Logger {
    level: Level,
}

impl Logger {
    pub fn new(level: Level) -> Self {
        Logger { level }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!("{} - {} - {} - {}", timestamp, record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

static INIT: Once = Once::new();
static mut LOGGER: Option<Logger> = None;

pub fn init(level: Level) -> Result<(), SetLoggerError> {
    unsafe {
        INIT.call_once(|| {
            LOGGER = Some(Logger::new(level));
        });

        if let Some(logger) = &LOGGER {
            log::set_logger(logger)
                .map(|()| log::set_max_level(LevelFilter::Trace))
        } else {
            Err(SetLoggerError::CannotSetLogger)
        }
    }
}
