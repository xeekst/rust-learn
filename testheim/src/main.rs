use std::io::{self, BufRead};
use std::{
    io::{BufReader, Error},
    os::windows::process::CommandExt,
    process::{Child, Command, Stdio},
    thread,
};

use anyhow::{anyhow, Ok, Result};
use futures::FutureExt;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

pub fn exec_windows_cmd_wait_output(cmd_with_args: &str) -> String {
    let child = Command::new("cmd")
        .arg("/c")
        .raw_arg(format!("chcp 65001 2>NUL 1>NUL && {}", cmd_with_args))
        .output()
        .unwrap();
    let buf = child.stdout;
    let text = std::str::from_utf8(&buf).unwrap();

    String::from(text)
}

const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;
const WINDOWS_DETACHED_PROCESS: u32 = 0x00000008;
const WINDOWS_CREATE_NEW_CONSOLE: u32 = 0x00000010;
pub fn exec_windows_cmd_without_cmdc_nowait(
    cmd: &str,
    args: &str,
    output_handler: fn(Result<String, Error>),
) -> Result<u32> {
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

fn handle_stdout_stderr(
    child: Child,
    cmd: &str,
    args: &str,
    output_handler: fn(Result<String, Error>),
) -> Result<()> {
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

#[tokio::main]
async fn main() {
    // loop{
    //     println!("Press enter to terminate the child thread");
    //     thread::sleep(Duration::from_millis(500));
    // }
    // return;
    println!("Press enter to terminate the child thread");
    let (tx, rx) = mpsc::channel();

    let t = tokio::task::spawn(async {
        println!("Working...");
        let r = exec_windows_cmd_without_cmdc_nowait(r#"D:\loopgo.exe"#, "", |e| {
            println!("out:{:?}", e.unwrap())
        });
        thread::sleep(Duration::from_millis(3500));
        let pid = r.unwrap();
        let ppp = heim::process::get(pid);
        if let std::result::Result::Ok(process) = heim::process::get(pid).await {
            process
                .kill()
                .await
                .map_err(|_| "Cannot kill process".to_string())
                .unwrap();
            println!(
                "{}",
                format!("Process with pid {} killed successfully!", pid)
            );
        } else {
            println!("{}", format!("Cannot get process with pid {}", pid));
        }

        panic!("normal exit");
    });

    // let t =  thread::spawn( move|| loop {
    //     println!("Working...");
    //     let r = exec_windows_cmd_without_cmdc_nowait(r#"D:\loopgo.exe"#, "", |e| {
    //         println!("out:{:?}", e.unwrap())
    //     });
    //     thread::sleep(Duration::from_millis(3500));

    //     panic!("normal exit");
    //     // match rx.try_recv() {
    //     //     Ok(_) | Err(TryRecvError::Disconnected) => {
    //     //         println!("Terminating.");
    //     //         break;
    //     //     }
    //     //     _ => (),
    //     // }
    // });
    loop {
        thread::sleep(Duration::from_millis(500));
        println!("is finish:{:?}", t.is_finished());
        if t.is_finished() {
            drop(t);
            break;
        }
    }
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);

    let _ = tx.send(());
}
