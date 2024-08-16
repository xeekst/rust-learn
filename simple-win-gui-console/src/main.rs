// #![windows_subsystem = "windows"]
fn main() {
    println!("Hello, world!");
    loop {
        
    }
}

// extern crate winapi;

// use std::ptr::null_mut;
// use std::io::{self, Write};
// use winapi::um::consoleapi::AllocConsole;
// use winapi::um::wincon::SetConsoleTitleA;
// use winapi::um::winnt::HANDLE;

// fn main() {
//     unsafe {
//         // 分配一个新的控制台窗口
//         AllocConsole();

//         // 设置控制台窗口标题
//         let title = "My Rust Console App";
//         SetConsoleTitleA(title.as_ptr() as *const i8);
//     }

//     println!("Hello, this is a custom console window!");

//     // 示例输入输出
//     let mut input = String::new();
//     print!("Please enter something: ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     println!("You entered: {}", input.trim());
// }
