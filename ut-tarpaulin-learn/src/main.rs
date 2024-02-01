mod functions;
mod unit_test;
#[cfg(not(tarpaulin_include))]
fn main() {
    add3(2, 4);
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub struct FtpServer {
    pub ip: String,
    pub port: u32,
    pub user: String,
    pub password: String,
}

pub fn add3(a: i32, b: i32) -> i32 {
    if a > 0 {
        a - b
    } else {
        a + b
    }
}

