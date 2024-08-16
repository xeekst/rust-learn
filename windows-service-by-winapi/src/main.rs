use rand::distributions::Alphanumeric;
use rand::Rng;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::io;
use std::iter::repeat;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{self, null_mut};
use std::{os::windows::ffi::OsStrExt, time::Duration};
use widestring::error::ContainsNul;
use widestring::{U16CString, WideCString, WideString};
use winapi::shared::minwindef::LPVOID;
use winapi::shared::winerror::{ERROR_INSUFFICIENT_BUFFER, ERROR_MORE_DATA};
use winapi::um::winnt::{
    SERVICE_AUTO_START, SERVICE_ERROR_NORMAL, SERVICE_WIN32, SERVICE_WIN32_OWN_PROCESS,
};
use winapi::um::winsvc::{
    ChangeServiceConfig2W, EnumServicesStatusExW, ENUM_SERVICE_STATUS_PROCESSW,
    LPENUM_SERVICE_STATUS_PROCESSW, QUERY_SERVICE_CONFIGW, SC_MANAGER_ALL_ACCESS,
    SC_STATUS_PROCESS_INFO, SERVICE_ALL_ACCESS, SERVICE_CONFIG_DESCRIPTION,
    SERVICE_CONTINUE_PENDING, SERVICE_PAUSED, SERVICE_PAUSE_PENDING, SERVICE_RUNNING,
    SERVICE_START_PENDING, SERVICE_STATE_ALL, SERVICE_STATUS_PROCESS, SERVICE_STOPPED,
    SERVICE_STOP_PENDING,
};
use winapi::{
    ctypes::c_void,
    shared::{
        minwindef::{DWORD, LPDWORD},
        ntdef::NULL,
    },
    um::{
        errhandlingapi::GetLastError,
        winbase::LocalFree,
        winnt::DELETE,
        winsvc::{
            CloseServiceHandle, ControlService, CreateServiceW, DeleteService, OpenSCManagerW,
            OpenServiceW, QueryServiceConfigW, QueryServiceStatus, StartServiceW, SC_HANDLE,
            SC_MANAGER_CONNECT, SC_MANAGER_CREATE_SERVICE, SC_MANAGER_ENUMERATE_SERVICE,
            SC_MANAGER_QUERY_LOCK_STATUS, SERVICE_CONTROL_STOP, SERVICE_QUERY_CONFIG,
            SERVICE_QUERY_STATUS, SERVICE_START, SERVICE_STOP,
        },
    },
};

fn open_sc_manager(desired_access: DWORD) -> Result<SC_HANDLE, DWORD> {
    let sc_manager_handle = unsafe { OpenSCManagerW(null_mut(), null_mut(), desired_access) };
    if sc_manager_handle.is_null() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(sc_manager_handle)
    }
}

mod utf16 {
    pub const DOUBLEQUOTE: u16 = '"' as u16;
    pub const BACKSLASH: u16 = '\\' as u16;
    pub const SPACE: u16 = ' ' as u16;
    pub const LINEFEED: u16 = '\n' as u16;
    pub const HTAB: u16 = '\t' as u16;
    pub const VTAB: u16 = 0x000B; // '\v'
}

pub struct Service {
    pub service_name: String,
    pub display_name: String,
    pub status: u32,
}

pub fn escape(s: Cow<'_, OsStr>) -> Cow<'_, OsStr> {
    static ESCAPE_CHARS: &[u16] = &[
        utf16::DOUBLEQUOTE,
        utf16::SPACE,
        utf16::LINEFEED,
        utf16::HTAB,
        utf16::VTAB,
    ];
    let needs_escape = s.is_empty() || s.encode_wide().any(|ref c| ESCAPE_CHARS.contains(c));
    if !needs_escape {
        return s;
    }

    let mut escaped_wide_string: Vec<u16> = Vec::with_capacity(s.len() + 2);
    escaped_wide_string.push(utf16::DOUBLEQUOTE);

    let mut chars = s.encode_wide().peekable();
    loop {
        let mut num_slashes = 0;
        while let Some(&utf16::BACKSLASH) = chars.peek() {
            chars.next();
            num_slashes += 1;
        }

        match chars.next() {
            Some(utf16::DOUBLEQUOTE) => {
                escaped_wide_string.extend(repeat(utf16::BACKSLASH).take(num_slashes * 2 + 1));
                escaped_wide_string.push(utf16::DOUBLEQUOTE);
            }
            Some(c) => {
                escaped_wide_string.extend(repeat(utf16::BACKSLASH).take(num_slashes));
                escaped_wide_string.push(c);
            }
            None => {
                escaped_wide_string.extend(repeat(utf16::BACKSLASH).take(num_slashes * 2));
                break;
            }
        }
    }

    escaped_wide_string.push(utf16::DOUBLEQUOTE);

    Cow::Owned(OsString::from_wide(&escaped_wide_string))
}

/// Escapes a given string, but also checks it does not contain any null bytes
fn escape_wide(s: impl AsRef<OsStr>) -> ::std::result::Result<WideString, ContainsNul<u16>> {
    let escaped = escape(Cow::Borrowed(s.as_ref()));
    let wide = WideCString::from_os_str(escaped)?;
    Ok(wide.to_ustring())
}

fn create_service(
    sc_manager: SC_HANDLE,
    service_name: &str,
    service_exe_path: &str,
    launch_args: Vec<OsString>,
) -> Result<(), DWORD> {
    let service_name_wstr = U16CString::from_str(service_name).unwrap();
    let executable_path = escape_wide(&service_exe_path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "executable path"))
        .unwrap();
    let mut launch_command_buffer = WideString::new();
    launch_command_buffer.push(executable_path);

    for (i, launch_argument) in launch_args.iter().enumerate() {
        let wide = escape_wide(launch_argument)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, format!("launch argument:{i}")))
            .unwrap();

        launch_command_buffer.push_str(" ");
        launch_command_buffer.push(wide);
    }
    let launch_command = unsafe { WideCString::from_ustr_unchecked(launch_command_buffer) };
    println!("launch_command:{:?}", launch_command);

    let service_handle = unsafe {
        CreateServiceW(
            sc_manager,
            service_name_wstr.as_ptr(),
            service_name_wstr.as_ptr(),
            SERVICE_ALL_ACCESS,
            SERVICE_WIN32_OWN_PROCESS,
            SERVICE_AUTO_START,
            SERVICE_ERROR_NORMAL,
            launch_command.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };

    let mut description =
        WideCString::from_str("XTEST AGENT SERVICE Your service description goes here.")
            .expect("Failed to create WideCString");

    let mut service_desc = winapi::um::winsvc::SERVICE_DESCRIPTIONA {
        lpDescription: description.as_mut_ptr() as *mut i8,
    };

    let change_config_result = unsafe {
        ChangeServiceConfig2W(
            service_handle,
            SERVICE_CONFIG_DESCRIPTION,
            &mut service_desc as *mut _ as LPVOID,
        )
    };

    if change_config_result == 0 {
        println!(
            "Failed to change service description. Error code: {}",
            unsafe { GetLastError() }
        );
    } else {
        println!("Service created successfully with description.");
    }

    if service_handle == std::ptr::null_mut() {
        println!("error:{:?}", std::io::Error::last_os_error());
        Err(unsafe { GetLastError() })
    } else {
        Ok(())
    }
}

fn delete_service(sc_manager_handle: SC_HANDLE, service_name: &str) -> Result<(), DWORD> {
    let service_name = WideCString::from_str(service_name).unwrap();
    let service_handle = unsafe {
        OpenServiceW(
            sc_manager_handle,
            service_name.as_ptr(),
            SC_MANAGER_CONNECT | DELETE,
        )
    };
    if service_handle.is_null() {
        println!("Failed to open service. Error code: {}", unsafe {
            GetLastError()
        });
        unsafe { CloseServiceHandle(sc_manager_handle) };
        return Err(unsafe { GetLastError() });
    }

    // 删除服务
    let delete_result = unsafe { DeleteService(service_handle) };
    if delete_result == 0 {
        println!("Failed to delete service. Error code: {}", unsafe {
            GetLastError()
        });

        return Err(unsafe { GetLastError() });
    } else {
        println!("Service deleted successfully.");

        Ok(())
    }
}

fn open_service(
    sc_manager_handle: SC_HANDLE,
    service_name: &str,
    desired_access: DWORD,
) -> Result<SC_HANDLE, DWORD> {
    let service_name_wstr = service_name
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let service_name_wstr = U16CString::from_str(service_name).unwrap();
    let service_handle = unsafe {
        OpenServiceW(
            sc_manager_handle,
            service_name_wstr.as_ptr(),
            desired_access,
        )
    };
    if service_handle.is_null() {
        println!("error:{:?}", std::io::Error::last_os_error());
        Err(unsafe { GetLastError() })
    } else {
        Ok(service_handle)
    }
}

// SERVICE_STOPPED: DWORD = 0x00000001;
// SERVICE_START_PENDING: DWORD = 0x00000002;
// SERVICE_STOP_PENDING: DWORD = 0x00000003;
// SERVICE_RUNNING: DWORD = 0x00000004;
// SERVICE_CONTINUE_PENDING: DWORD = 0x00000005;
// SERVICE_PAUSE_PENDING: DWORD = 0x00000006;
// SERVICE_PAUSED: DWORD = 0x00000007;
fn query_service_status(service_handle: SC_HANDLE) -> Result<DWORD, DWORD> {
    let mut service_status = unsafe { std::mem::zeroed() };
    let result = unsafe { QueryServiceStatus(service_handle, &mut service_status) };
    if result == 0 {
        Err(unsafe { GetLastError() })
    } else {
        Ok(service_status.dwCurrentState)
    }
}

fn start_service(service_handle: SC_HANDLE) -> Result<(), DWORD> {
    let result = unsafe { StartServiceW(service_handle, 0, null_mut()) };
    if result == 0 {
        Err(unsafe { GetLastError() })
    } else {
        Ok(())
    }
}

fn stop_service(service_handle: SC_HANDLE) -> Result<(), DWORD> {
    let mut service_status = unsafe { std::mem::zeroed() };
    let result =
        unsafe { ControlService(service_handle, SERVICE_CONTROL_STOP, &mut service_status) };
    if result == 0 {
        Err(unsafe { GetLastError() })
    } else {
        Ok(())
    }
}

fn query_all_service(sc_manager: SC_HANDLE) -> Result<(), DWORD> {
    let mut bytes_needed: DWORD = 0;
    let mut services_returned: DWORD = 0;
    let mut resume_handle: DWORD = 0;
    let mut service_status: Vec<u8> = Vec::with_capacity(1);

    // 初次调用以获取需要的缓冲区大小
    let success = unsafe {
        EnumServicesStatusExW(
            sc_manager,
            SC_STATUS_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            null_mut(),
            0,
            &mut bytes_needed,
            &mut services_returned,
            &mut resume_handle,
            null_mut(),
        )
    };

    let last_error = unsafe { GetLastError() };
    if success == 0 && last_error != ERROR_MORE_DATA {
        return Err(last_error);
    }

    // 分配适当大小的缓冲区并再次调用
    service_status.resize(bytes_needed as usize, 0);

    let success = unsafe {
        EnumServicesStatusExW(
            sc_manager,
            SC_STATUS_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            service_status.as_mut_ptr() as *mut u8,
            bytes_needed,
            &mut bytes_needed,
            &mut services_returned,
            &mut resume_handle,
            null_mut(),
        )
    };

    let last_error = unsafe { GetLastError() };
    if success == 0 && last_error != ERROR_MORE_DATA {
        return Err(last_error);
    }

    // 解析服务信息并打印服务名称
    let services = service_status.as_ptr() as *const ENUM_SERVICE_STATUS_PROCESSW;
    for i in 0..services_returned {
        let service = unsafe { &*services.add(i as usize) };
        let service_name = unsafe { WideCString::from_ptr_str(service.lpServiceName) };
        let display_name = unsafe { WideCString::from_ptr_str(service.lpDisplayName) };
        let status = match service.ServiceStatusProcess.dwCurrentState {
            SERVICE_STOPPED => "STOPPED",
            SERVICE_START_PENDING => "START_PENDING",
            SERVICE_STOP_PENDING => "STOP_PENDING",
            SERVICE_RUNNING => "RUNNING",
            SERVICE_CONTINUE_PENDING => "CONTINUE_PENDING",
            SERVICE_PAUSE_PENDING => "PAUSE_PENDING",
            SERVICE_PAUSED => "PAUSED",
            _ => "Unknown",
        };
        println!(
            "name: {} | display: {} | status: {status} ",
            service_name.to_string_lossy(),
            display_name.to_string_lossy()
        );
    }

    Ok(())
}

// fn delete_service(service_handle: SC_HANDLE) -> Result<(), DWORD> {
//     let result = unsafe { DeleteService(service_handle) };
//     if result == 0 {
//         Err(unsafe { GetLastError() })
//     } else {
//         Ok(())
//     }
// }

fn query_service_config(service_handle: SC_HANDLE) -> Result<(), DWORD> {
    // 查询服务配置
    let mut bytes_needed: DWORD = 0;
    unsafe { QueryServiceConfigW(service_handle, null_mut(), 0, &mut bytes_needed) };
    let mut buffer: Vec<u8> = Vec::with_capacity(bytes_needed as usize);
    let service_config: *mut QUERY_SERVICE_CONFIGW =
        buffer.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW;

    if unsafe {
        QueryServiceConfigW(
            service_handle,
            service_config,
            bytes_needed,
            &mut bytes_needed,
        ) == 0
    } {
        return Err(unsafe { GetLastError() });
    }

    unsafe { buffer.set_len(bytes_needed as usize) };

    // 处理并显示服务配置信息
    let service_config_ref: &QUERY_SERVICE_CONFIGW = unsafe { &*service_config };
    println!("Service Type: {}", service_config_ref.dwServiceType);
    println!("Start Type: {}", service_config_ref.dwStartType);
    println!("Error Control: {}", service_config_ref.dwErrorControl);

    if !service_config_ref.lpBinaryPathName.is_null() {
        let binary_path = unsafe { U16CString::from_ptr_str(service_config_ref.lpBinaryPathName) }
            .to_string_lossy();
        println!("Binary Path: {}", binary_path);
    }

    if !service_config_ref.lpLoadOrderGroup.is_null() {
        let load_order_group = utf16_ptr_to_string(service_config_ref.lpLoadOrderGroup);
        println!("Load Order Group: {}", load_order_group);
    }

    if !service_config_ref.lpDependencies.is_null() {
        let dependencies = utf16_ptr_to_string(service_config_ref.lpDependencies);
        println!("Dependencies: {}", dependencies);
    }

    if !service_config_ref.lpServiceStartName.is_null() {
        let service_start_name = utf16_ptr_to_string(service_config_ref.lpServiceStartName);
        println!("Service Start Name: {}", service_start_name);
    }

    if !service_config_ref.lpDisplayName.is_null() {
        let display_name = utf16_ptr_to_string(service_config_ref.lpDisplayName);
        println!("Display Name: {}", display_name);
    }

    Ok(())
}

// 辅助函数: 将 UTF-16 指针转换为 Rust 字符串
fn utf16_ptr_to_string(ptr: *const u16) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let len = unsafe { (0..).take_while(|&i| *ptr.offset(i) != 0).count() };
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    String::from_utf16_lossy(slice)
}

fn print_status(status: DWORD) {
    match status {
        SERVICE_STOPPED => println!("Service is stopped."),
        SERVICE_START_PENDING => println!("Service is starting."),
        SERVICE_STOP_PENDING => println!("Service is stopping."),
        SERVICE_RUNNING => println!("Service is running."),
        SERVICE_CONTINUE_PENDING => println!("Service is continuing."),
        SERVICE_PAUSE_PENDING => println!("Service is pausing."),
        SERVICE_PAUSED => println!("Service is paused."),
        _ => println!("Unknown service state."),
    }
}

fn main() {
    let service_exe_path =
        r#"D:\codes\rust-learn\windows-service-rust\target\debug\windows-service-rust.exe"#;
    // let mut rng = rand::thread_rng();

    let list: Vec<Box<dyn FnOnce()>> = Vec::new();
    // // 生成一个长度为 255 的随机字符串
    // let random_string: String = (0..255).map(|_| rng.sample(Alphanumeric) as char).collect();
    // let s1 = random_string
    //     .encode_utf16()
    //     .chain(Some(0))
    //     .collect::<Vec<u16>>();
    // let s2 = unsafe { U16CString::from_str_unchecked(random_string) };
    // let r = s1 == s2.clone().into_vec_with_nul();
    // println!("s1:\n{:?} \ns2:\n{:?}\n  => {}", s1, s2.into_vec_with_nul(), r);

    // return;

    let sc_manager_handle = open_sc_manager(SC_MANAGER_ALL_ACCESS).unwrap();
    // delete_service(sc_manager_handle, "xtest-service-rust").unwrap();
    // return;
    query_all_service(sc_manager_handle).unwrap();
    return;
    create_service(
        sc_manager_handle,
        "xtest-service-rust",
        service_exe_path,
        vec![OsString::from("arg1"), OsString::from("arg2")],
    )
    .unwrap();

    return;
    let service_name = "uvnc_service";
    let service_handle = open_service(
        sc_manager_handle,
        service_name,
        SERVICE_QUERY_STATUS | SERVICE_START | SERVICE_STOP | DELETE | SERVICE_QUERY_CONFIG,
    )
    .unwrap();

    //query_service_config(service_handle).unwrap();

    let status = query_service_status(service_handle).unwrap();
    print_status(status);

    // Start the service
    match start_service(service_handle) {
        Ok(_) => println!("Service started successfully."),
        Err(error) => println!("Failed to start service. Error: {}", error),
    }

    std::thread::sleep(Duration::from_secs(1));
    let status = query_service_status(service_handle).unwrap();
    print_status(status);

    // Stop the service
    match stop_service(service_handle) {
        Ok(_) => println!("Service stopped successfully."),
        Err(error) => println!("Failed to stop service. Error: {}", error),
    }

    std::thread::sleep(Duration::from_secs(1));
    let status = query_service_status(service_handle).unwrap();
    print_status(status);

    // //Delete the service
    // match delete_service(service_handle) {
    //     Ok(_) => println!("Service deleted successfully."),
    //     Err(error) => println!("Failed to delete service. Error: {}", error),
    // }

    // Close the service and SCM handles
    unsafe {
        CloseServiceHandle(service_handle);
        CloseServiceHandle(sc_manager_handle);
    }
    println!("Hello, world!");
}
