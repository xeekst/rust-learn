use std::io::BufRead;

use expectrl::{spawn, Eof, Error, Regex};

fn main() -> Result<(), Error> {
    let mut p = spawn("ftp 127.0.0.1")?;
    let mut buf = String::from("");
    println!("line: {:?},{:?}", p.read_line(&mut buf), buf);
    p.expect(Regex("Name \\(.*\\):"))?;
    p.send_line("anonymous")?;
    p.expect("Password")?;
    p.send_line("test")?;
    p.expect("ftp>")?;
    p.send_line("cd upload")?;
    p.expect("successfully changed.\r\nftp>")?;
    p.send_line("pwd")?;
    p.expect(Regex("[0-9]+ \"/upload\""))?;
    p.send_line("exit")?;
    p.expect(Eof)?;
    //assert_eq!(p.wait(Some(3000)), Ok(WaitStatus::Exited(p.pid(), 0)));
    Ok(())
}
