use std::ffi::OsStr;
use std::ffi::c_void;
use std::path::Path;
use std::ptr;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::PSTR;
use windows::Win32::Foundation::PWSTR;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Threading::CreateProcessW;
use windows::Win32::System::Threading::PROCESS_CREATION_FLAGS;
use windows::Win32::System::Threading::PROCESS_INFORMATION;
use windows::Win32::System::Threading::STARTUPINFOW;

fn main() {
    println!("Hello, world!");
    let command = Path::new(r#"D:\codes\rust-learn\print_loop\target\debug\print_loop.exe"#).as_os_str();
    let cur_dir = Path::new(r#"D:\codes\rust-learn\print_loop\target\debug"#);
    create_process(command,false,Some(cur_dir));
    loop {
        
    }
}
fn create_process(
    command: &OsStr,
    inherit_handles: bool,
    current_directory: Option<&Path>,
)  {
    let mut startup_info = STARTUPINFOW::default();
    let mut process_info = PROCESS_INFORMATION::default();

    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

    let process_creation_flags = PROCESS_CREATION_FLAGS(0);

    let res = unsafe {
        if let Some(directory) = current_directory {
            let directory = directory.as_os_str();
            windows::Win32::System::Threading::CreateProcessW(
                PWSTR::default(),
                command,
                std::ptr::null() as *const SECURITY_ATTRIBUTES,
                std::ptr::null() as *const SECURITY_ATTRIBUTES,
                inherit_handles,
                windows::Win32::System::Threading::CREATE_NEW_CONSOLE,
                std::ptr::null() as *const c_void,
                directory,
                &startup_info,
                &mut process_info as *mut PROCESS_INFORMATION,
            )
        } else {
            windows::Win32::System::Threading::CreateProcessW(
                PWSTR::default(),
                command,
                std::ptr::null() as *const SECURITY_ATTRIBUTES,
                std::ptr::null() as *const SECURITY_ATTRIBUTES,
                inherit_handles,
                process_creation_flags,
                std::ptr::null() as *const c_void,
                PWSTR::default(),
                &startup_info,
                &mut process_info as *mut PROCESS_INFORMATION,
            )
        }
    };

    if res.as_bool() {
        println!("process_info:{:?}",process_info);
    } else {
        println!("error:{:?}",unsafe { GetLastError().0 });
    }
}
