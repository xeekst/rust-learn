use core::time;
use std::thread;

fn main() {
    loop {
        println!("Hello, world!");
        thread::sleep(time::Duration::from_secs(3));
    }
}
