use std::env;

fn main() {
    println!("cargo:warning=Target:{}", env::var("TARGET").unwrap());
    println!("cargo:warning=Profile:{}", env::var("PROFILE").unwrap());
}
