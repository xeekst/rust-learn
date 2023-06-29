use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;
use std::ptr;

use fltk::app;
use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;

fn main() {
    //not work
    let app = app::App::default();
    let mut window = Window::default().with_size(1100, 600);
    let group = Group::default().size_of_parent().center_of_parent();
    window.end();
    window.show();
    // 获取 Group 的句柄
    let p_hadnle = window.raw_handle() as HANDLE;
    println!("p_hadnle:{:?}", p_hadnle as u32);
    let h = exec_bin_newwindow_nowait_by_win32("cmd.exe", "", "./", p_hadnle);
    println!("h:{:?}", h as u32);
    let result = unsafe {
        let hmenu_child = unsafe { GetMenu(859572 as HWND) };
        // 移除子窗口的菜单
        println!("hmenu_child:{:?}", hmenu_child);

        let r = SetParent(662752 as HWND, p_hadnle as HWND);
        println!("SetParent:{:?}", r);
        //ShowWindow(h as HWND, SW_SHOW);
    };

    app.run().unwrap();
}

use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HWND;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::GetMenu;
use winapi::um::winuser::SetParent;
use winapi::um::winuser::ShowWindow;
use winapi::um::winuser::SW_SHOW;
use winapi::um::winuser::WS_CHILD;
use winapi::um::winuser::WS_VISIBLE;
pub fn exec_bin_newwindow_nowait_by_win32(
    bin: &str,
    args: &str,
    workspace_abs_dir: &str,
    h: HANDLE,
) -> HANDLE {
    use std::ptr;
    use winapi::shared::minwindef::DWORD;

    use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
    use winapi::um::winbase::CREATE_NEW_CONSOLE;

    let workspace_wide_ptr = if workspace_abs_dir == "" {
        ptr::null_mut()
    } else {
        workspace_abs_dir
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr() as winapi::shared::ntdef::LPCWSTR
    };

    let command_line = format!("{} {}", bin, args);
    let command_line_wide: Vec<u16> = command_line.encode_utf16().chain(Some(0)).collect();

    let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
    startup_info.dwFlags = winapi::um::winbase::STARTF_USESHOWWINDOW;
    startup_info.wShowWindow = winapi::um::winuser::SW_SHOW as u16;
    //startup_info.hStdInput = h as HANDLE;

    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let result: winapi::shared::minwindef::BOOL;
    unsafe {
        result = CreateProcessW(
            ptr::null(),                            // lpApplicationName
            command_line_wide.as_ptr() as *mut u16, // lpCommandLine
            ptr::null_mut(),                        // lpProcessAttributes
            ptr::null_mut(),                        // lpThreadAttributes
            winapi::shared::minwindef::FALSE,       // bInheritHandles
            CREATE_NEW_CONSOLE,                     // dwCreationFlags
            ptr::null_mut(),                        // lpEnvironment
            workspace_wide_ptr,                     // lpCurrentDirectory
            &mut startup_info,                      // lpStartupInfo
            &mut process_info,                      // lpProcessInformation
        );
    }

    if result == winapi::shared::minwindef::TRUE {
        process_info.hProcess
    } else {
        panic!(
            "CreateProcessW error: cmd: {bin}, args: {args}, workspace_abs_dir: {workspace_abs_dir}, error code:{result}"
        );
    }
}
