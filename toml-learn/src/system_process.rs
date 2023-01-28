use anyhow::{anyhow, Ok, Result};
use std::{
    io::{BufRead, BufReader, Error},
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    thread,
};

//https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;
const WINDOWS_DETACHED_PROCESS: u32 = 0x00000008;
const WINDOWS_CREATE_NEW_CONSOLE: u32 = 0x00000010;

pub fn exec_windows_cmd_wait_output(cmd: &str) -> Result<String> {
    let child = Command::new("cmd").args(["/C", cmd]).output()?;
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
pub fn exec_windows_cmd_nowait(cmd: &str, output_handler: fn(Result<String, Error>)) -> Result<()> {
    let child = Command::new("cmd")
        .args(["/C", cmd])
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

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
        None => return Err(anyhow!("{} child.stdout is None", cmd)),
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
        None => return Err(anyhow!("{} child.stderr is None", cmd)),
    }

    Ok(())
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
