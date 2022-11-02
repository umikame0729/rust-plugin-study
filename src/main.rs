use lib::Return;
use log::{info, set_logger, set_max_level, Level, Log};

fn main() -> Return {
    set_max_level(log::LevelFilter::Trace);
    set_logger(&AppLog {}).unwrap();

    info!("Hello, world!");

    lib::module_load()?;
    Ok(())
}

struct AppLog {}

impl Log for AppLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() > Level::Trace
    }

    fn log(&self, record: &log::Record) {
        println!("[{}] {}", record.target(), record.args());
    }

    fn flush(&self) {}
}
