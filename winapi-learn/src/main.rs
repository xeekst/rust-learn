extern crate winapi;
use winapi::CW_USEDEFAULT;
use winapi::DWORD;
use winapi::HBRUSH;
use winapi::HCURSOR;
use winapi::HICON;
use winapi::HINSTANCE;
use winapi::HMENU;
use winapi::HWND;
use winapi::LPARAM;
use winapi::LPCWSTR;
use winapi::LRESULT;
use winapi::SW_SHOWNORMAL;
use winapi::UINT;
use winapi::WPARAM;
use winapi::WS_EX_CLIENTEDGE;

use winapi::winuser::WNDCLASSW;
use winapi::winuser::WS_OVERLAPPEDWINDOW;
use winapi::winuser::WS_VISIBLE;

extern crate user32;
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

fn to_wide(msg: &str) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    wide
}

pub unsafe extern "system" fn win_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if msg == winapi::winuser::WM_DESTROY {
        user32::PostQuitMessage(0);
    }
    return user32::DefWindowProcW(h_wnd, msg, w_param, l_param);
}

fn main() {
    let class_name = to_wide("my_window");
    let icon = unsafe { user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) };
    let cursor = unsafe { user32::LoadCursorW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) };
    let wnd = WNDCLASSW {
        style: 0,
        lpfnWndProc: Some(win_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0 as HINSTANCE,
        hIcon: icon,
        hCursor: cursor,
        hbrBackground: 16 as HBRUSH,
        lpszMenuName: 0 as LPCWSTR,
        lpszClassName: class_name.as_ptr(),
    };
    let ret = unsafe { user32::RegisterClassW(&wnd) };
    if ret == 0 {
        let msg = to_wide("register failed.");
        unsafe {
            user32::MessageBoxW(null_mut(), msg.as_ptr(), msg.as_ptr(), winapi::MB_OK);
        }
    }
    let h_wnd_desktop = unsafe {};
    unsafe {
        let h_wnd_desktop = user32::CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1400,
            700,
            user32::GetDesktopWindow(),
            0 as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        user32::ShowWindow(h_wnd_desktop, SW_SHOWNORMAL);
        user32::UpdateWindow(h_wnd_desktop);
        let hwnd = user32::CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            400,
            h_wnd_desktop,
            0 as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        user32::ShowWindow(hwnd, SW_SHOWNORMAL);
        user32::UpdateWindow(hwnd);

        let mut msg = winapi::winuser::MSG {
            hwnd: 0 as HWND,
            message: 0 as UINT,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as DWORD,
            pt: winapi::windef::POINT { x: 0, y: 0 },
        };

        println!("init the msg structure success.");
        // Finally we run the standard application loop -
        loop {
            println!("get msg from the queue");
            let pm = user32::GetMessageW(&mut msg, hwnd, 0, 0);
            println!("msg received.");
            if pm > 0 {
                user32::TranslateMessage(&mut msg);
                user32::DispatchMessageW(&mut msg);
            }
        }
    }
}
