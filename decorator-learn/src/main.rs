use anyhow::{anyhow, Result};
use macros::{deco, logging_proc};

fn main() {
    println!("Hello, world! re_add:{}", re_add(2, 3).unwrap());
    // let f = logging(add);
    // let r = f(33).unwrap();

    println!("add_de:{}", add_de(22));
}

#[logging_proc]
pub fn add_de(a: i32) -> i32 {
    println!("step 2222:{a}");

    a + 1
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

use adorn::adorn;

#[adorn(retry)]
fn re_add(a: u8, b: u8) -> Result<u8> {
    //let c = a + b;
    Ok(a + b)
}

fn retry<F>(f: F, a: u8, b: u8) -> Result<u8>
where
    F: Fn(u8, u8) -> Result<u8>,
{
    println!("retry before");
    let r = f(a, b);
    println!("retry after");

    r
}
