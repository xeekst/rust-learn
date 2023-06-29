use std::ptr;
use std::time::Duration;

use winapi::um::winbase::{PowerSetRequest, SetThreadExecutionState};
use winapi::um::winnt::{
    PowerRequestSystemRequired, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
    EXECUTION_STATE,
};
use winapi::um::winuser::{
    PostMessageW, SendMessageW, SystemParametersInfoW, HWND_BROADCAST, SC_MONITORPOWER,
    SC_SCREENSAVE, SPI_SETSCREENSAVEACTIVE, WM_SYSCOMMAND,
};

fn main() {
    println!("start");
    set_windows_display_on();
    set_windows_system_busy();
    println!("end");
    loop {
        
    }
}

fn set_windows_system_busy() {
    std::thread::spawn(|| {
        loop {
            unsafe {
                // 阻止系统休眠
                SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED);
            }
            std::thread::sleep(Duration::from_secs(5));
        }
    });
}

fn set_windows_display_on() {
    std::thread::spawn(|| {
        loop {
            unsafe {
                // 控制显示器休眠状态
                SendMessageW(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, -1);
            }
            std::thread::sleep(Duration::from_secs(5));
        }
    });
}
