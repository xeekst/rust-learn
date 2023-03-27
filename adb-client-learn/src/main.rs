
use std::net::Ipv4Addr;

fn main() {
    // let mut connexion = AdbTcpConnexion::new(Ipv4Addr::from([10, 176, 121, 49]), 7405).unwrap();
    // connexion.shell_command(None, vec!["df", "-h"]);
    let mut connexion = mozdevice::adb::DeviceSerial::new();
    println!("devices:{:?}", connexion.devices());
    println!("Hello, world!");
}
