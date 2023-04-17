extern crate suppaftp;

use suppaftp::list::File;
use suppaftp::native_tls::{TlsConnector, TlsStream};
use suppaftp::FtpStream;

fn main() {
    let ftp_stream = FtpStream::connect("10.176.36.40:990").unwrap();
    // Switch to the secure mode
    let tls = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build().unwrap();
    let mut ftp_stream = ftp_stream.into_secure(tls, "need-replace").unwrap();
    ftp_stream.login("ftps_automation", "1qasw@").unwrap();
    let files: Vec<File> = ftp_stream.list(None).ok().unwrap().iter().map(|x| File::try_from(x.as_str()).ok().unwrap()).collect();

    for f in files {
        println!("{:?}", f);
    }
    // Do all public stuff
    assert!(ftp_stream.quit().is_ok());

    println!("Hello, world!");
}
