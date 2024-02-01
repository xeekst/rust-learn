extern crate winapi;

use std::process::Command;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::HANDLE;
use winapi::um::wlanapi::*;

const ERROR_SUCCESS: u32 = 0;
fn main() {
    //1. 先刷新WIFI
    //2. 找一台电脑，连接该WIFI，然后查看profile:netsh wlan show profile
    //3. 导出该配置文件，保存下来: netsh wlan export profile "moto X40_1013"
    //4. 使用该配置文件，在需要连接wifi的机器上使用命令导入 profile : netsh wlan add profile filename="Wi-Fi-moto X40_1013.xml"
    //5. 连接指定的WiFi: netsh wlan connect ssid="moto X40_1013" name="moto X40_1013"
    unsafe {
        let mut client_handle: HANDLE = std::ptr::null_mut();
        let mut version: DWORD = 0;

        // Initialize WLAN API
        let result = WlanOpenHandle(
            2, // WLAN_API_VERSION_2_0
            std::ptr::null_mut(),
            &mut version,
            &mut client_handle,
        );

        if result == ERROR_SUCCESS {
            println!("WLAN API initialized successfully");

            // Scan for available networks
            let mut interface_list: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
            let result =
                WlanEnumInterfaces(client_handle, std::ptr::null_mut(), &mut interface_list);

            if result == ERROR_SUCCESS {
                for i in 0..(*interface_list).dwNumberOfItems as usize {
                    let interface = &(*interface_list).InterfaceInfo[i];
                    println!(
                        "Scanning on interface: {:?}",
                        interface.strInterfaceDescription
                    );

                    
                    let scan_result = WlanScan(
                        client_handle,
                        &interface.InterfaceGuid,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );

                    if scan_result == ERROR_SUCCESS {
                        println!("Scan initiated successfully");
                    } else {
                        println!("Failed to initiate scan: {}", scan_result);
                    }
                }

                WlanFreeMemory(interface_list as *mut _);
            } else {
                println!("Failed to enumerate WLAN interfaces: {}", result);
            }

            WlanCloseHandle(client_handle, std::ptr::null_mut());
        } else {
            println!("Failed to initialize WLAN API: {}", result);
        }
    }
    let buf = Command::new("netsh")
        .args(&["wlan", "show", "networks"])
        .output()
        .unwrap()
        .stdout;
    let text = std::str::from_utf8(&buf).unwrap();

    println!("wifi:{text}");
}

// extern crate winapi;

// use std::process::Command;
// use std::ptr;
// use winapi::shared::guiddef::GUID;
// use winapi::shared::minwindef::{DWORD, LPVOID};
// use winapi::shared::wlantypes::DOT11_SSID;
// use winapi::um::winnt::HANDLE;
// use winapi::um::wlanapi::*;

// const ERROR_SUCCESS: u32 = 0;
// fn main() {
//     //todo!("not work, 87 error");
//     unsafe {
//         let mut client_handle: HANDLE = ptr::null_mut();
//         let mut version: DWORD = 0;

//         // Initialize WLAN API
//         let result = WlanOpenHandle(
//             WLAN_API_VERSION_2_0, // WLAN_API_VERSION_2_0
//             ptr::null_mut(),
//             &mut version,
//             &mut client_handle,
//         );

//         if result == ERROR_SUCCESS as u32 {
//             println!("WLAN API initialized successfully");

//             let profile_name: Vec<u16> = "X40".encode_utf16().chain(Some(0)).collect();

//             let mut connect_params: WLAN_CONNECTION_PARAMETERS = std::mem::zeroed();
//             connect_params.wlanConnectionMode = wlan_connection_mode_profile; // Use the constant directly
//             connect_params.strProfile = profile_name.as_ptr() as *const u16;
//             //connect_params.pDot11Ssid = ptr::null_mut();
//             let ssid_bytes: [u8; 14] = [
//                 b'm', b'o', b't', b'o', b' ', b'X', b'4', b'0', b'_', b'1', b'0', b'1', b'3', 0x00,
//             ];
//             let mut ssid: DOT11_SSID = std::mem::zeroed();
//             ssid.uSSIDLength = ssid_bytes.len() as u32;
//             for i in 0..ssid_bytes.len() {
//                 ssid.ucSSID[i] = ssid_bytes[i];
//             }

//             connect_params.pDot11Ssid = &mut ssid;
//             connect_params.pDesiredBssidList = ptr::null_mut();
//             connect_params.dwFlags = 0;

//             let result = WlanConnect(
//                 client_handle,
//                 ptr::null_mut(),
//                 &mut connect_params,
//                 ptr::null_mut(),
//             );

//             if result == ERROR_SUCCESS as u32 {
//                 println!("Connected to Wi-Fi network");
//             } else {
//                 println!("Failed to connect to Wi-Fi network: {}", result);
//             }

//             WlanCloseHandle(client_handle, ptr::null_mut());
//         } else {
//             println!("Failed to initialize WLAN API: {}", result);
//         }
//     }
// }
