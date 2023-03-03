use ssh2::Session;
use std::{io::Read, net::TcpStream, time, path::Path, thread};
fn main() {
    //Connect to the local SSH server
    test_exec_command();
}

fn test_exec_command() {
    let tcp = TcpStream::connect("10.176.60.55:4024").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    println!("start handshake");
    sess.handshake().unwrap();
    println!("start auth");
    sess.userauth_password("atm-web", "Aa123456").unwrap();
    println!("sess.authenticated():{}", sess.authenticated());
    let mut channel = sess.channel_session().unwrap();
    channel.exec("dir").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());

    if let Ok(stream) = TcpStream::connect("10.176.60.55:4024") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}


fn test_sftp() {
    let tcp = TcpStream::connect("10.176.60.55:4024").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    println!("[{:?}]start handshake", time::SystemTime::now());
    sess.handshake().unwrap();

    println!("start auth");
    sess.userauth_password("ftpuser", "1qasw@").unwrap();
    println!("sess.authenticated():{}", sess.authenticated());
    let sftp = sess.sftp().unwrap();
    println!("list:{:#?}", sftp.readdir(Path::new("/")).unwrap());
    thread::sleep(time::Duration::from_secs(10));
}