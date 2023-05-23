use std::{
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    process::{Child, Command, Stdio},
    thread,
};

fn main() {
    exec_cmd_nowindow_nocmdc_nowait(
        r#"D:\codes\rust-learn\call_process2\target\debug\call_process2.exe"#,
        "",
        |s| println!("{:?}", s),
    );
    // exec_cmd_newwindow_cmdc_nowait(
    //     r#"D:\codes\rust-learn\call_process2\target\debug\call_process2.exe"#,
    //     "",
    //     r#"D:\codes\rust-learn\call_process2\target\debug"#,
    // );
    loop {}
}

pub fn exec_cmd_newwindow_cmdc_nowait(cmd: &str, args: &str, working_dir: &str) {
    let mut command = Command::new(cmd);
    let command = command.current_dir(working_dir);
    let child = command
        .raw_arg(args)
        .creation_flags(0x00000010)
        .spawn()
        .unwrap();
    let id = &child.id();
}

pub fn exec_cmd_nowindow_nocmdc_nowait(cmd: &str, args: &str, output_handler: fn(String)) {
    let child = Command::new(cmd)
        .raw_arg(args)
        .creation_flags(0x08000000)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let pid = child.id();
    let _ = handle_stdout_stderr(child, cmd, args, output_handler);
}

fn handle_stdout_stderr(child: Child, cmd: &str, args: &str, output_handler: fn(String)) {
    let child_out = child.stdout.unwrap();
    thread::spawn(move || {
        let reader = BufReader::new(child_out);
        for line in reader.lines() {
            output_handler(line.unwrap())
        }
    });

    let child_out = child.stderr.unwrap();
    thread::spawn(move || {
        let reader = BufReader::new(child_out);
        for line in reader.lines() {
            output_handler(line.unwrap())
        }
    });
}
