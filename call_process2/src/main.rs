use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::path::PathBuf;
use std::ptr;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPVOID, TRUE};
use winapi::shared::ntdef::{LPCWSTR, LPWSTR};
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::CREATE_NEW_CONSOLE;
use std::os::windows::ffi::OsStrExt;

fn main() {
    let command = r#"D:\codes\rust-learn\print_loop\target\debug\print_loop.exe"#; // 替换为实际的可执行文件名
                                                                                   // let command_line = format!("{}", command);
                                                                                   // let command_line_wide: Vec<u16> = command_line.encode_utf16().chain(Some(0)).collect();

    // let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    // startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
    // startup_info.dwFlags = winapi::um::winbase::STARTF_USESHOWWINDOW;
    // startup_info.wShowWindow = winapi::um::winuser::SW_SHOW as u16;

    // let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    // let result: BOOL;
    // unsafe {
    //     result = CreateProcessW(
    //         ptr::null(),                            // lpApplicationName
    //         command_line_wide.as_ptr() as *mut u16, // lpCommandLine
    //         ptr::null_mut(),                        // lpProcessAttributes
    //         ptr::null_mut(),                        // lpThreadAttributes
    //         FALSE,                                  // bInheritHandles
    //         CREATE_NEW_CONSOLE,                     // dwCreationFlags
    //         ptr::null_mut(),                        // lpEnvironment
    //         ptr::null_mut(),                        // lpCurrentDirectory
    //         &mut startup_info,                      // lpStartupInfo
    //         &mut process_info,                      // lpProcessInformation
    //     );
    // }

    // if result == TRUE {
    //     println!("create print loop:{}", process_info.dwProcessId);
    //     println!("子进程成功启动");
    // } else {
    //     eprintln!("启动子进程失败，错误码: ");
    // }
    println!("path:{}", PathBuf::from(command).exists());
    exec_bin_newwindow_nowait_by_win32(
        command,
        "",
        r#"D:\codes\rust-learn\print_loop\target\debug"#,
    )
    .unwrap();
    loop {}
}

pub fn exec_bin_newwindow_nowait_by_win32(
    bin: &str,
    args: &str,
    workspace_abs_dir: &str,
) -> Result<u32> {
    use std::ptr;
    use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
    use winapi::um::winbase::CREATE_NEW_CONSOLE;

    let workspace_wide_ptr = if workspace_abs_dir.is_empty() {
        ptr::null_mut()
    } else {
        // let r = std::fs::read_dir(workspace_abs_dir)?;
        // for item in r {

        // }
        // let ws = workspace_abs_dir
        //     .encode_utf16()
        //     .chain(Some(0))
        //     .collect::<Vec<u16>>();
        // println!("ws:{:?}", ws);
        // ws.as_ptr() as winapi::um::winnt::LPCWSTR

        workspace_abs_dir
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr() as winapi::um::winnt::LPCWSTR

        // let wks = std::path::Path::new(workspace_abs_dir).as_os_str();
        // std::os::windows::prelude::OsStrExt::encode_wide(wks).collect::<Vec<u16>>().as_ptr() as winapi::um::winnt::LPCWSTR
    };
    let workspace_wide = workspace_abs_dir
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let wwide: Vec<u16> = OsStr::new(workspace_abs_dir).encode_wide().chain(once(0)).collect();
    let command_line = format!("{bin} {args}");
    let cwide: Vec<u16> = OsStr::new(workspace_abs_dir).encode_wide().chain(once(0)).collect();
    println!("command_line:{command_line}");
    let command_line_wide = command_line
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
    startup_info.dwFlags = winapi::um::winbase::STARTF_USESHOWWINDOW;
    startup_info.wShowWindow = winapi::um::winuser::SW_SHOW as u16;

    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let result: winapi::shared::minwindef::BOOL;
    //https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
    unsafe {
        result = CreateProcessW(
            ptr::null(),                            // lpApplicationName
            command_line_wide.as_ptr() as *mut u16, // lpCommandLine 一定要 mut 类型，由于win32 api 的要求
            ptr::null_mut(),                        // lpProcessAttributes
            ptr::null_mut(),                        // lpThreadAttributes
            winapi::shared::minwindef::FALSE,       // bInheritHandles
            CREATE_NEW_CONSOLE,                     // dwCreationFlags
            ptr::null_mut(),                        // lpEnvironment
            wwide.as_ptr() as *mut u16,    // lpCurrentDirectory
            &mut startup_info,                      // lpStartupInfo
            &mut process_info,                      // lpProcessInformation
        );
    }

    if result == winapi::shared::minwindef::TRUE {
        Ok(process_info.dwProcessId)
    } else {
        let error = Error::last_os_error();

        Err(anyhow!(
            "CreateProcessW error: cmd: {bin}, args: {args}, workspace_abs_dir: {workspace_abs_dir}, error result:{result},raw_os_error code:{:?}, msg:{:?}",
            error.raw_os_error(),
            error,
        ))
    }
}

pub fn exec_cmd_newwindow_cmdc_nowait(cmd: &str, args: &str, working_dir: &str) {
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

    if result == TRUE {
        println!("create print loop:{}", process_info.dwProcessId);
        println!("子进程成功启动");
    } else {
        eprintln!("启动子进程失败，错误码: ");
    }
}
