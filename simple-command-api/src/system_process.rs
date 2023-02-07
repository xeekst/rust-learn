use anyhow::{anyhow, Ok, Result};
use log::{debug, error, info, warn};
use std::{
    io::{BufRead, BufReader, Error},
    os::windows::process::CommandExt,
    process::{Child, Command, Stdio},
    thread,
};

#[cfg(target_os = "windows")]
use winapi::shared::minwindef::DWORD;
#[cfg(target_os = "windows")]
use winapi::shared::ntdef::HANDLE;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, LPPROCESSENTRY32, PROCESSENTRY32, TH32CS_SNAPPROCESS};
#[cfg(target_os = "windows")]
#[cfg(target_os = "windows")]
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE};

use crate::encoding_config;

#[derive(Debug)]
pub struct ProcessInformation {
    pub pid: u32,
    pub name: String,
}

impl ProcessInformation {
    fn new(_pid: u32, _name: String) -> ProcessInformation {
        ProcessInformation { pid: _pid, name: _name }
    }
}

pub struct ProcessInformationIterator {
    process_information: ProcessInformation,
    index: usize,
    process_snapshot: HANDLE,
    process_entry: PROCESSENTRY32,
}
struct Process(HANDLE);
impl Process {
    fn open(pid: DWORD) -> Result<Process> {
        // https://msdn.microsoft.com/en-us/library/windows/desktop/ms684320%28v=vs.85%29.aspx
        let pc = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_TERMINATE, 0, pid) };
        if pc == std::ptr::null_mut() {
            return Err(anyhow!("!OpenProcess"));
        }
        Ok(Process(pc))
    }

    fn kill(self) -> Result<()> {
        unsafe { TerminateProcess(self.0, 1) };
        Ok(())
    }
}
impl Drop for Process {
    fn drop(&mut self) {
        unsafe { winapi::um::handleapi::CloseHandle(self.0) };
    }
}

//https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;
const WINDOWS_DETACHED_PROCESS: u32 = 0x00000008;
const WINDOWS_CREATE_NEW_CONSOLE: u32 = 0x00000010;

pub fn exec_cmd_newwindow_wait_output(cmd_with_args: &str) -> Result<String> {
    let child = Command::new("cmd")
        .arg("/c")
        .creation_flags(WINDOWS_CREATE_NEW_CONSOLE)
        .raw_arg(format!("chcp 65001 2>NUL 1>NUL && {}", cmd_with_args))
        .spawn()?;
    //let id = child.id();
    //println!("id:{}", id);
    // thread::spawn(move || {
    //     println!("id:{}", id);
    //     thread::sleep(std::time::Duration::from_millis(3500));
    //     kill_process(id).unwrap();
    // });
    let child = child.wait_with_output()?;
    let buf = child.stdout;
    let text = std::str::from_utf8(&buf)?;

    Ok(String::from(text))
}

pub fn exec_cmd_nowindow_without_cmdc_nowait(cmd: &str, args: &str, output_handler: fn(Result<String, Error>)) -> Result<u32> {
    let child = Command::new(cmd)
        .raw_arg(args)
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let pid = child.id();
    let _ = handle_stdout_stderr(child, cmd, args, output_handler)?;

    Ok(pid)
}

pub fn exec_cmd_nowindow_wait_output(cmd_with_args: &str) -> Result<String> {
    let child = Command::new("cmd")
        .arg("/c")
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .raw_arg(format!("chcp 65001 2>NUL 1>NUL && {}", cmd_with_args))
        .output()?;

    let buf = child.stdout;
    let stderr = String::from_utf8_lossy(&child.stderr);
    if !stderr.is_empty() {
        error!("stderr:{:?}", stderr);
    }

    let text = String::from_utf8_lossy(&buf);
    Ok(text.into_owned().to_string())
}

pub fn exec_cmd_nowindow_wait_output_use_codepage_encoding(cmd_with_args: &str) -> Result<String> {
    let child = Command::new("cmd")
        .arg("/c")
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .raw_arg(cmd_with_args)
        .output()?;
    let buf = child.stdout;
    let stderr = String::from_utf8_lossy(&child.stderr);
    if !stderr.is_empty() {
        error!("stderr:{:?}", stderr);
    }

    let codepage_buf = Command::new("cmd")
        .arg("/c")
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .raw_arg("chcp")
        .output()?
        .stdout;

    let codepages = String::from_utf8_lossy(&codepage_buf);
    let codepages = codepages.split(":").collect::<Vec<&str>>();
    let codepage = match codepages.get(1) {
        Some(c) => c.replace(" ", "").replace("\r\n", "").replace(".", ""),
        None => {
            warn!("can not found windows cmd codepage ,will be use 65001 as default");

            "65001".to_string()
        }
    };
    let codepage = codepage.parse::<u16>()?;
    let encoding = match encoding_config::encoding_from_codepage(codepage) {
        Some(d) => d,
        None => {
            error!("codepage:{codepage} not support, please add codepage map encoding, will be use default: UTF_8_INIT..");

            &encoding_rs::UTF_8_INIT
        }
    };
    let (text, enc, had_errors) = encoding.decode(&buf);
    if had_errors {
        error!("cmd_with_args:{cmd_with_args}, encoding:{:?}, decode text error. source vec<u8>:{:?}", enc, buf);
    }

    Ok(text.to_string())
}

pub fn exec_cmd_nowindow_wait_output2(cmd_with_args: &str) -> Result<String> {
    let mut command = Command::new("cmd");
    let command = command.current_dir(std::env::current_dir().unwrap());
    let child = command
        .arg("/c")
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .raw_arg(format!("chcp 65001 2>NUL 1>NUL && {}", cmd_with_args))
        .output()?;

    let buf = child.stdout;
    let text = std::str::from_utf8(&buf)?;

    info!(
        "+++++++++++++++++++++++++++++++++++ workdir:{:?} ++++++++++++++++++++++++++++++++++++++++++++",
        command.get_current_dir()
    );
    info!(
        "current exe:{:?}\r\ncurrent dir:{:?}\r\nabs_dir    :{:?}",
        std::env::current_exe(),
        std::env::current_dir(),
        std::fs::canonicalize(std::path::Path::new("./"))
    );
    Ok(String::from(text))
}

/// rust 格式的 cmd 是不会解释处理 单双引号、空格
pub fn exec_rust_windows_cmd_wait_output(cmd_with_args: &str) -> Result<String> {
    let child = Command::new("cmd").args(["/c", "chcp 65001 2>NUL 1>NUL &&", cmd_with_args]).output()?;
    let buf = child.stdout;
    let text = std::str::from_utf8(&buf)?;

    Ok(String::from(text))
}

/// `exec_windows_cmd_nowait` 在windows上调用cmd程序来执行命令, 并使用一个闭包来处理输出
///
/// # Examples
///
/// ```
/// # use core::time;
/// # use agent_core::{system_process};
/// # use std::thread;
///
/// let cmd = r#"D:\codes\rust_learn\print_loop\target\debug\print_loop.exe"#;
/// let _ = system_process::exec_windows_cmd_nowait("echo 2 & TIMEOUT /T 3 & echo 3 & TIMEOUT /T 3 & ECHO END", |s|{ println!("{:?}",s) });
/// thread::sleep(time::Duration::from_secs(10));
/// ```
pub fn exec_cmd_nowindow_nowait(cmd_with_args: &str, output_handler: fn(Result<String, Error>)) -> Result<u32> {
    let child = Command::new("cmd")
        .args(["/C", cmd_with_args])
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let id = &child.id();

    let _ = handle_stdout_stderr(child, cmd_with_args, "", output_handler)?;

    Ok(*id)
}

pub fn exec_cmd_without_cmdc_nowait(cmd: &str, args: &str, output_handler: fn(Result<String, Error>)) -> Result<u32> {
    let child = Command::new(cmd)
        .raw_arg(args)
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let id = &child.id();

    let _ = handle_stdout_stderr(child, cmd, args, output_handler)?;

    Ok(*id)
}

pub fn exec_cmd_newwindow_cmdc_nowait(cmd: &str, args: &str, working_dir: &str) -> Result<u32> {
    let mut command = Command::new(cmd);
    let command = command.current_dir(working_dir);
    let child = command.raw_arg(args).creation_flags(WINDOWS_CREATE_NEW_CONSOLE).spawn()?;
    let id = &child.id();

    //let _ = handle_stdout_stderr(child, cmd, args, output_handler)?;

    Ok(*id)
}

fn handle_stdout_stderr(child: Child, cmd: &str, args: &str, output_handler: fn(Result<String, Error>)) -> Result<()> {
    match child.stdout {
        Some(child_out) => {
            thread::spawn(move || {
                let reader = BufReader::new(child_out);
                for line in reader.lines() {
                    output_handler(line)
                }
                // 非阻塞每次读取缓存区所有行
                //let mut line = String::new();
                // loop {
                //     reader.read_line(&mut line).unwrap();
                //     println!("{}", line);
                // }
            });
        }
        None => return Err(anyhow!("{:?} {:?} child.stdout is None", cmd, args)),
    }

    match child.stderr {
        Some(child_out) => {
            thread::spawn(move || {
                let reader = BufReader::new(child_out);
                for line in reader.lines() {
                    output_handler(line)
                }
            });
        }
        None => return Err(anyhow!("{:?} {:?} child.stderr is None", cmd, args)),
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn kill_process(pid: u32) -> Result<()> {
    let pc = Process::open(pid as DWORD)?;
    pc.kill()?;

    Ok(())
}

fn char_arr_to_string(chars: &[i8]) -> String {
    chars.into_iter().map(|c| *c as u8 as char).collect()
}

#[cfg(target_os = "windows")]
impl ProcessInformationIterator {
    pub fn new() -> ProcessInformationIterator {
        let h_process_snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if h_process_snapshot == INVALID_HANDLE_VALUE {
            error!("Invalid handle value");
        }
        //debug!("ProcessInformationIterator: Got process snapshot handle, continue.");
        let mut pe: PROCESSENTRY32;
        unsafe {
            pe = ::std::mem::zeroed();
        }
        let a = ::std::mem::size_of::<PROCESSENTRY32>();

        let lppe: LPPROCESSENTRY32 = &mut pe;
        pe.dwSize = a as u32;
        let res = unsafe { Process32First(h_process_snapshot, lppe) };
        if res == 0 {
            panic!("Can't get process list");
        }

        let pid: u32 = pe.th32ProcessID;
        let process_name: String = char_arr_to_string(&pe.szExeFile);
        ProcessInformationIterator {
            process_information: ProcessInformation::new(pid, process_name),
            index: 0,
            process_snapshot: h_process_snapshot,
            process_entry: pe,
        }
    }
}

#[cfg(target_os = "windows")]
impl Iterator for ProcessInformationIterator {
    type Item = ProcessInformation;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.index = self.index + 1;
        if self.index == 1 {
            return Some(ProcessInformation::new(self.process_information.pid, self.process_information.name.clone()));
        }

        let mut pe = self.process_entry;
        let lppe = &mut pe;
        let res;
        unsafe {
            (*lppe).szExeFile = ::std::mem::zeroed();
            res = Process32Next(self.process_snapshot, lppe);
        }
        if res != 1 {
            // No more processes, finish the iteration
            None
        } else {
            let pid: u32 = (*lppe).th32ProcessID;
            let process_name: String = char_arr_to_string(&(*lppe).szExeFile);
            Some(ProcessInformation::new(pid, process_name))
        }
    }
}

pub fn exec_linux_sh_wait_output(sh: &str) -> Result<String> {
    let output = Command::new("sh").arg("-c").arg(sh).output()?;
    let buf = output.stdout;
    let text = std::str::from_utf8(&buf)?;

    Ok(String::from(text))
}

pub fn exec_linux_sh_nowait(cmd: &str, output_handler: fn(Result<String, Error>)) -> Result<()> {
    todo!()
}
