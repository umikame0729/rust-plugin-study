use std::path::PathBuf;

use log::{info, Log, Record};

pub fn module_load() -> Return {
    let dirs = [".", "target/debug", "target/release"];

    for dir in dirs
        .iter()
        .map(|f| PathBuf::from(f))
        .filter(|f| f.exists() && f.is_dir())
    {
        for file in dir
            .read_dir()?
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap().path())
            .filter(|f| f.is_file())
            .filter(|f| {
                f.extension().is_some()
                    && ["dll", "so"].contains(&f.extension().unwrap().to_str().unwrap())
            })
            .filter(|f| f.file_stem().unwrap().ne("lib"))
        {
            info!("load file: {}", file.display());
            info!("load by loading...");
            load_libloading(&file)?;
        }
    }
    Ok(())
}

fn load_libloading(path: &PathBuf) -> Return {
    unsafe {
        let lib = libloading::Library::new(path)?;
        {
            let get_name = lib.get::<fn() -> &'static str>(b"name")?;
            let name = get_name();
            info!("name = {}", name);
        }
        {
            let call_cb = lib.get::<fn(fn(String))>(b"test_cb")?;
            call_cb(test_cb);
        }
        {
            let log_init = lib.get::<fn(fn(&Record)) -> Return>(b"log_init")?;
            log_init(log_out)?;
        }
    }
    Ok(())
}

pub type Return<T = ()> = anyhow::Result<T>;

fn test_cb(s: String) {
    println!("test_cb: {}", s);
}

fn log_out(record: &Record) {
    log::logger().log(record);
}

pub struct LibLogger {
    pub output: Option<fn(&Record)>,
}

impl Log for LibLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        (self.output.unwrap())(record)
    }

    fn flush(&self) {}
}
