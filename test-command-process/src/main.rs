use std::{env, path::Path};

fn main() {
    println!("Hello, world!");
    println!(
        "current exe:{:?}\r\ncurrent dir:{:?}\r\nabs_dir    :{:?}",
        env::current_exe(),
        env::current_dir(),
        std::fs::canonicalize(Path::new("./"))
    );

    loop {}
}
