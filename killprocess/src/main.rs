#[macro_use]
extern crate gstuff;

use std::process;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::HANDLE;
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE};

struct Process(HANDLE);
impl Process {
    fn open(pid: DWORD) -> Result<Process, String> {
        // https://msdn.microsoft.com/en-us/library/windows/desktop/ms684320%28v=vs.85%29.aspx
        let pc = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_TERMINATE, 0, pid) };
        if pc == null_mut() {
            return ERR!("!OpenProcess");
        }
        Ok(Process(pc))
    }

    fn kill(self) -> Result<(), String> {
        unsafe { TerminateProcess(self.0, 1) };
        Ok(())
    }
}
impl Drop for Process {
    fn drop(&mut self) {
        unsafe { winapi::um::handleapi::CloseHandle(self.0) };
    }
}

fn main() {
    //system("taskkill /F /T /IM program.exe");
    let child = process::Command::new("ping")
        .arg("8.8.8.8")
        .arg("-t")
        .spawn()
        .expect("Couldn't run 'ping'");

    let pid = child.id();
    let pc = Process::open(pid as DWORD).expect("!open");
    println!("Process {} opened.", pid);
    thread::sleep(Duration::from_secs(5));
    pc.kill().expect("!kill");
    println!("Process {} killed.", pid);
}




// use std::{process, thread, time::Duration};

// fn main() {
//     let mut child = process::Command::new("ping")
//         .arg("8.8.8.8")
//         .arg("-t")
//         .spawn()
//         .expect("Couldn't run 'ping'");

//     thread::sleep(Duration::from_secs(5));
//     child.kill().expect("!kill");
// }