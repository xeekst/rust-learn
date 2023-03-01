use ssh_jumper::{
    model::{HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use std::{
    borrow::Cow,
    net::{IpAddr, Ipv4Addr},
};

#[tokio::main]
async fn main() {
    let jump_host = HostAddress::IpAddr(IpAddr::V4(Ipv4Addr::new(10, 10, 10, 55)));
    let jump_host_auth_params =
        JumpHostAuthParams::password(Cow::Borrowed("********"), Cow::Borrowed("*******"));
    let target_socket = HostSocketParams {
        address: HostAddress::HostName(Cow::Borrowed("127.0.0.1")),
        port: 3389,
    };
    let ssh_params = SshTunnelParams::new(jump_host, jump_host_auth_params, target_socket)
        // Optional: OS will allocate a port if this is left out
        .with_local_port(33855)
        .with_jump_host_port(4024);
    SshJumper::open_tunnel(&ssh_params).await.unwrap();

    println!("Hello, world!");
}
