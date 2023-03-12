use anyhow::{anyhow, Result};
use macros::{deco, logging_proc};

fn main() {
    println!("Hello, world!");
    let f = logging(add);
    let r = f(33).unwrap();

    add_de();
}

#[logging_proc]
pub fn add_de() -> () {
    println!("step 2222");
    
    ()
}

pub fn add(i: i32) -> Result<i32> {
    Ok(1 + 2 + i)
}

pub fn logging<F>(func: F) -> impl Fn(i32) -> Result<i32>
where
    F: Fn(i32) -> Result<i32>,
{
    move |i| {
        println!("Input = {}", i);
        let out = func(i);
        println!("Output = {:?}", out);
        out
    }
}
