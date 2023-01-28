// use std::{
//     fs::{self, File},
//     io::Read,
//     sync::RwLock,
// };

// use lazy_static::lazy_static;
// use serde_derive::Deserialize;
// use toml::Value;

// lazy_static! {
//     pub static ref ATM_SERVICE_ADDRESS: RwLock<String> =
//         RwLock::new("http://127.0.0.1:9000".to_string());
// }

// #[derive(Debug, Deserialize)]
// struct ConfigTest {
//     ip: String,
//     port: Option<u16>,
//     keys: Keys,
// }

// #[derive(Debug, Deserialize)]
// struct Keys {
//     github: String,
//     travis: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// struct Config {
//     server_01: Server,
// }

// #[derive(Debug, Deserialize)]
// struct Server {
//     ip: String,
//     port: u32,
//     user: String,
//     password: String,
//     exportports: Vec<u32>,
//     sftp: Sftp,
// }

// #[derive(Debug, Deserialize)]
// struct Sftp {
//     user: String,
//     password: String,
// }

// fn main() {
//     {
//         println!("{}", ATM_SERVICE_ADDRESS.read().unwrap().to_string());
//         println!("{}", ATM_SERVICE_ADDRESS.read().unwrap().to_string());
//     }
//     {
//         let mut addr = ATM_SERVICE_ADDRESS.write().unwrap();
//         *addr = "assign new".to_string();
//     }
//     {
//         println!("{}", ATM_SERVICE_ADDRESS.read().unwrap());
//     }

//     return;

//     let text = fs::read_to_string("test.toml").unwrap();
//     let dict = text.parse::<Value>().unwrap();
//     println!("dict:{:#?}", dict);

//     let config_toml: Config =
//         toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap();

//     println!("config.toml:{:#?}", config_toml);

//     let config: ConfigTest = toml::from_str(
//         r#"
//         ip = '127.0.0.1'

//         [keys]
//         github = 'xxxxxxxxxxxxxxxxx'
//         travis = 'yyyyyyyyyyyyyyyyy'
//     "#,
//     )
//     .unwrap();

//     println!("config:{:?}", config);
//     assert_eq!(config.ip, "127.0.0.1");
//     assert_eq!(config.port, None);
//     assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
//     assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
// }

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::format,
    fs,
    path::{Path, PathBuf},
    process::Command, thread, time::{self, Duration},
};
use wmi::{COMLibrary, Variant, WMIConnection, WMIDateTime};
mod system_process;
mod tests;

fn test_copy_dir() {
    let mut options = CopyOptions::new();
    options.content_only = true;
    options.copy_inside = true;

    let r = fs_extra::dir::copy(PathBuf::from("x1"), PathBuf::from("x2"), &options);
    println!(
        "{:?}, {}",
        r,
        PathBuf::from("x1").into_os_string().into_string().unwrap()
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{
        println!("goooo");
        thread::sleep(Duration::from_secs(3));
    }

    test_xml();

    let s = format!(r#"C:\OneDriveTemp -r "ss" {0} \{1} {2}"#, 23, 12, 42);
    if Path::new(s.as_str()).exists() {
        println!("ttttttttt {}", s);
    }
    println!("{}", s);
    return Ok(());
    // let com_con = COMLibrary::new()?;
    // let wmi_con = WMIConnection::new(com_con.into())?;

    // let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_OperatingSystem")?;

    // for os in results {
    //     println!("{:#?}", os);
    // }

    // #[derive(Deserialize, Debug)]
    // struct Win32_OperatingSystem {
    //     Caption: String,
    //     Name: String,
    //     CurrentTimeZone: i16,
    //     Debug: bool,
    //     EncryptionLevel: u32,
    //     ForegroundApplicationBoost: u8,
    //     LastBootUpTime: WMIDateTime,
    // }

    // let results: Vec<Win32_OperatingSystem> = wmi_con.query()?;

    // for os in results {
    //     println!("{:#?}", os);
    // }
    let mut t = system_process::exec_windows_cmd_wait_output("wmic cpu get Name")?;
    println!("{:?}", t);
    let tmp = t.split("\r\r\n");
    let vec: Vec<&str> = tmp.collect();
    println!("{:?}", vec);
    let name = vec[1];
    println!("{:?}", name.trim());
    Ok(())
}

fn test_xml() {
    Command::new("cmd").args(["/C", "echo \"sss\""]);

    let xml = r#"<?xml version="1.0"?>
    <Configuration>
        <CheckPoint>0</CheckPoint>
        <ITCode>luotao9</ITCode>
        <TestPlanName>tt</TestPlanName>
        <TaskID></TaskID>
        <Case Order = "1" >
            <CaseID>18</CaseID>
            <CaseName>OS_PowerOptions_15_DTTPWS_RestorePowerSettingsFunction</CaseName>
            <Filter>test=~/PPA_100115_OSPowerOptions15.*/</Filter>
            <PlanTime>1</PlanTime>
            <ActualTime>0</ActualTime>
            <Reboot>NO</Reboot>
        </Case>
        <Case Order = "2" >
            <CaseID>22</CaseID>
            <CaseName>OS_TaskBarCheck_010_TaskbarIcon_Win10</CaseName>
            <Filter>test=~/PPA_20000_OSTaskBarCheck.*/</Filter>
            <PlanTime>1</PlanTime>
            <ActualTime>0</ActualTime>
            <Reboot>NO</Reboot>
        </Case>
    </Configuration>
    "#;
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Configuration {
        #[serde(rename = "$unflatten=CheckPoint")]
        check_point: usize,
        #[serde(rename = "$unflatten=ITCode")]
        itcode: String,
        #[serde(rename = "$unflatten=TestPlanName")]
        testplan_name: String,
        #[serde(rename = "Case", default)]
        cases: Vec<Case>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Case {
        #[serde(rename = "$unflatten=CaseID")]
        case_id: i32,
        #[serde(rename = "$unflatten=CaseName")]
        case_name: String,
        #[serde(rename = "Order")]
        order: i32,
        #[serde(rename = "$unflatten=Filter")]
        filter: String,
        #[serde(rename = "$unflatten=PlanTime")]
        plan_times: i32,
        #[serde(rename = "$unflatten=ActualTime")]
        actual_times: i32,
        #[serde(rename = "$unflatten=Reboot")]
        reboot: bool,
    }
    let r: Configuration = quick_xml::de::from_str(xml).unwrap();

    println!("{:#?}", r);
    let str = quick_xml::se::to_string(&r).unwrap();
    fs::write(Path::new("xml.xml"), &str).unwrap();
    println!("Serialize:{:?}", &str);

    for i in 0..10 {
        println!("{}", i);
    }
}
