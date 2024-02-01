use std::env;

use anyhow::Result;

fn main() {
    println!("Hello, world!:{:?}", env::var("RUST_BACKTRACE"));

    env::set_var("RUST_BACKTRACE", "1");
    //env::set_var("RUST_BACKTRACE", "1");
    println!("error:{:?}", env::var("RUST_BACKTRACE"));
    //panic!("asg");
    if let Err(e) = test_anyhow() {
        println!("e:{:?} ", e);
    }
}

fn test_anyhow() -> Result<i32> {
    test_anyhow2()
}

fn test_anyhow2() -> Result<i32> {
    test_anyhow3()
}

fn test_anyhow3() -> Result<i32> {
    test_anyhow4()
}

fn test_anyhow4() -> Result<i32> {
    return Err(anyhow::anyhow!("error !!!!!!!!!"));

    Ok(2)
}
