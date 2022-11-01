use std::path::PathBuf;

pub fn module_load() {
    let dirs = [".", "target/debug", "target/release"];

    for dir in dirs
        .iter()
        .map(|f| PathBuf::from(f))
        .filter(|f| f.exists() && f.is_dir())
    {
        for file in dir
            .read_dir()
            .unwrap()
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap().path())
            .filter(|f| f.is_file())
            .filter(|f| {
                f.extension().is_some()
                    && ["dll", "so"].contains(&f.extension().unwrap().to_str().unwrap())
            })
            .filter(|f| f.file_stem().unwrap().ne("lib"))
        {
            println!("load file: {}", file.display());
            println!("load by loading...");
            load_libloading(&file);
            println!("load by dlopen2...");
            load_dlopen2(&file);
        }
    }
}

fn load_libloading(path: &PathBuf) {}
fn load_dlopen2(path: &PathBuf) {}
