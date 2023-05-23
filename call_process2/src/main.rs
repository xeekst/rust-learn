// use std::process::Command;
// use winapi::shared::minwindef::{DWORD, TRUE, BOOL, FALSE};
// use winapi::um::processthreadsapi::{CreateProcessW, STARTUPINFOW, PROCESS_INFORMATION};
// use winapi::um::winbase::{CREATE_NEW_CONSOLE, CREATE_UNICODE_ENVIRONMENT, STARTF_USESHOWWINDOW};
// use winapi::um::wincon::{CreateConsoleScreenBuffer, SetConsoleActiveScreenBuffer};
// use winapi::um::winnt::{HANDLE};
// use winapi::um::winuser::{SW_SHOWNORMAL, SW_SHOW};

// fn main() {
//     let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
//     startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
//     startup_info.dwFlags = STARTF_USESHOWWINDOW;
//     startup_info.wShowWindow = SW_SHOW as u16;

//     let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

//     let result = unsafe {
//         CreateProcessW(
//             std::ptr::null_mut(),
//             r#"D:\codes\rust-learn\print_loop\target\debug\print_loop.exe"#.as_ptr() as *mut u16,
//             std::ptr::null_mut(),
//             std::ptr::null_mut(),
//             FALSE,
//             CREATE_NEW_CONSOLE,
//             std::ptr::null_mut(),
//             std::ptr::null_mut(),
//             &mut startup_info,
//             &mut process_info,
//         )
//     };
//     if result == TRUE {
//         println!("子进程 p3 已成功启动并在新控制台窗口中运行。");
//     } else {
//         eprintln!("启动子进程 p3 失败。");
//     }
//     // exec_cmd_newwindow_cmdc_nowait(
//     //     r#"D:\codes\rust-learn\print_loop\target\debug\print_loop.exe"#,
//     //     "",
//     //     r#"D:\codes\rust-learn\print_loop\target\debug"#,
//     // );
//     loop {}
// }

// // pub fn exec_cmd_newwindow_cmdc_nowait(cmd: &str, args: &str, working_dir: &str) {
// //     // STARTUPINFOEXA
// //     // let mut startup_info = STARTUPINFOA::default();
// //     // startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;
// //     // startup_info.dwFlags = 0x00000001; // STARTF_USESHOWWINDOW
// //     // startup_info.wShowWindow = SW_SHOW as u16;

// //     let mut command = Command::new(cmd);
// //     let command = command.current_dir(working_dir);
// //     let child = command
// //         .raw_arg(args)
// //         .creation_flags(0x00000010)
// //         .startup_info()
// //         .spawn()
// //         .unwrap();
// //     let id = &child.id();
// // }

use std::ptr;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPVOID, TRUE};
use winapi::shared::ntdef::{LPCWSTR, LPWSTR};
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::CREATE_NEW_CONSOLE;

fn main() {
    let command = r#"D:\codes\rust-learn\print_loop\target\debug\print_loop.exe"#; // 替换为实际的可执行文件名

    let command_line = format!("{}", command);
    let command_line_wide: Vec<u16> = command_line.encode_utf16().chain(Some(0)).collect();

    let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
    startup_info.dwFlags = winapi::um::winbase::STARTF_USESHOWWINDOW;
    startup_info.wShowWindow = winapi::um::winuser::SW_SHOW as u16;

    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let result: BOOL;
    unsafe {
        result = CreateProcessW(
            ptr::null(),                            // lpApplicationName
            command_line_wide.as_ptr() as *mut u16, // lpCommandLine
            ptr::null_mut(),                        // lpProcessAttributes
            ptr::null_mut(),                        // lpThreadAttributes
            FALSE,                                  // bInheritHandles
            CREATE_NEW_CONSOLE,                     // dwCreationFlags
            ptr::null_mut(),                        // lpEnvironment
            ptr::null_mut(),                        // lpCurrentDirectory
            &mut startup_info,                      // lpStartupInfo
            &mut process_info,                      // lpProcessInformation
        );
    }

    if result == TRUE {
        println!("create print loop:{}",process_info.dwProcessId);
        println!("子进程成功启动");
    } else {
        eprintln!("启动子进程失败，错误码: ");
    }
    loop {}
}

pub fn exec_cmd_newwindow_cmdc_nowait(cmd: &str, args: &str, working_dir: &str) -> i32 {
    let command_line = format!("{} {}", cmd, args);
    let command_line_wide: Vec<u16> = command_line.encode_utf16().chain(Some(0)).collect();
    let workspace_cstring = std::ffi::CString::new(working_dir).expect("Failed to create CString");

    let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
    startup_info.dwFlags = winapi::um::winbase::STARTF_USESHOWWINDOW;
    startup_info.wShowWindow = winapi::um::winuser::SW_SHOW as u16;

    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let result: BOOL;
    unsafe {
        result = CreateProcessW(
            ptr::null(),                            // lpApplicationName
            command_line_wide.as_ptr() as *mut u16, // lpCommandLine
            ptr::null_mut(),                        // lpProcessAttributes
            ptr::null_mut(),                        // lpThreadAttributes
            FALSE,                                  // bInheritHandles
            CREATE_NEW_CONSOLE,                     // dwCreationFlags
            ptr::null_mut(),                        // lpEnvironment
            workspace_cstring.as_ptr() as LPWSTR,   // lpCurrentDirectory
            &mut startup_info,                      // lpStartupInfo
            &mut process_info,                      // lpProcessInformation
        );
    }

    result
}
