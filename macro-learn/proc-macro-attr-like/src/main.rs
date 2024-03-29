use std::fmt::Debug;
use test_attr_macroinner::{print_tokenstream, testprint};
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

#[testprint(name = "wow", times = 3)]
fn fp<T: Debug>(v: T)
where
    T: Sized,
{
    println!("just stringfy:{:?}", v);
}

#[print_tokenstream]
fn fn_v1<F>(name: String, idx: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    let i2 = f(idx);

    i2
}

#[derive(Debug)]
struct S;

fn main() {
    fp("parammmmmmmm");
    five_times!(2 + 2);

    let v = fn_v1("s".to_string(), 33, |i: i32| i + 1);
    println!("Hello, world!:{v}");
}
