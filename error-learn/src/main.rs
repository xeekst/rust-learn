use std::env;

use anyhow::Result;

fn main() {
    println!("Hello, world!:{:?}",env::var("RUST_BACKTRACE"));

    env::set_var("RUST_BACKTRACE", "full");
    //env::set_var("RUST_BACKTRACE", "1");
    println!("error:{:?}",env::var("RUST_BACKTRACE"));
    //panic!("asg");
    if let Err(e) = test_anyhow() {
        println!("e:{:?} , anyhow backtrace:{:#?}", e, e.backtrace());
    }
}

fn test_anyhow() -> Result<i32> {
    return Err(anyhow::anyhow!("error !!!!!!!!!"));

    Ok(2)
}
