// fn main() {
//     use std::io::Write;
//     use std::process::{Command, Stdio};

//     let mut child = Command::new("cmd")
//         .arg("/C ssh -N -L 5505:127.0.0.1:5502 t@192.168.0.103")
//         //.arg("-N -L 5505:127.0.0.1:5502 t@127.0.0.1")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to spawn child process");

//     let mut stdin = child.stdin().expect("Failed to open stdin");
//     std::thread::spawn(move || {
//         stdin
//             .write_all("1".as_bytes())
//             .expect("Failed to write to stdin");
//     });

//     let output = child.wait_with_output().expect("Failed to read stdout");
//     println!("out:{:?}", output);
// }
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args = [
        "-N",
        "-o",
        "ServerAliveInterval=60",
        "-o",
        "ServerAliveCountMax=3",
        "-L",
        &format!("{}:{}", "5505", "127.0.0.1:5502"),
        "t@192.168.0.103",
    ];
    let mut child = 
        //Command::new("loop-print.exe")
        Command::new("ssh.exe")
        //.args(&args)
        .arg(" -NTC -L 5505:127.0.0.1:5502 t@192.168.0.103")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // let child_stdin = child.stdin.as_mut().unwrap();
    // child_stdin.write_all(b"yes\n")?;
    // child_stdin.flush().unwrap();
    // Close stdin to finish and avoid indefinite blocking
    //drop(child_stdin);

    //let output = child.wait_with_output()?;
    std::thread::sleep(Duration::from_secs(2));
    match child.stdout.take() {
        Some(child_out) => {
            thread::spawn(move || {
                let reader = BufReader::new(child_out);
                for line in reader.lines() {
                    println!("out:{:?}", line);
                }
            });
        }
        None => println!(" child.stdout is None"),
    }
    match child.stderr.take() {
        Some(stderr) => {
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    println!("stderr:{:?}", line);
                }
            });
        }
        None => println!(" child.stderr is None"),
    }
    std::thread::sleep(Duration::from_secs(10));
    println!("output = ");

    
    Ok(())
}
