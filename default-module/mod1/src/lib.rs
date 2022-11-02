use anyhow::anyhow;
use lib::{LibLogger, Return};
use log::{info, set_logger, set_max_level, Record};

#[no_mangle]
pub fn name() -> &'static str {
    "dsknflsd"
}

#[no_mangle]
pub fn test_cb(cb: fn(String)) {
    cb("nmp[[pgr".to_owned());
}

static mut LOGGER: LibLogger = LibLogger { output: None };

#[no_mangle]
pub fn log_init(output: fn(&Record)) -> Return {
    unsafe {
        LOGGER.output = Some(output);
    }

    set_max_level(log::LevelFilter::Trace);
    unsafe {
        if let Err(err) = set_logger(&LOGGER) {
            return Err(anyhow!("{}", err.to_string()));
        }
    }

    info!("log init test");

    Ok(())
}
