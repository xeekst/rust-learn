use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, Command, Stdio},
    sync::RwLock,
    thread::{self, JoinHandle, Thread},
    time::Duration,
};

use anyhow::Result;
use expectrl::{spawn, Session};

//#[derive(Debug)]
pub struct SSHTunnel {
    id: String,
    name: String,
    forward_port: String,
    dst_host_port: String,
    ssh_user_server_port: String,
    ssh: Option<Session>,
}

impl SSHTunnel {
    pub fn new(
        id: &str,
        name: &str,
        forward_port: &str,
        dst_host_port: &str,
        ssh_user_server_port: &str,
    ) -> SSHTunnel {
        SSHTunnel {
            id: id.to_string(),
            name: name.to_string(),
            forward_port: forward_port.to_string(),
            dst_host_port: dst_host_port.to_string(),
            ssh_user_server_port: ssh_user_server_port.to_string(),
            ssh: Option::None,
        }
    }

    pub fn start_tunnel(&mut self) -> Result<()> {
        let command = format!(
            "ssh -N -o ServerAliveInterval=60 -o ServerAliveCountMax=3 -L {0}:{1} {2}",
            self.forward_port, self.dst_host_port, self.ssh_user_server_port
        );

        let mut ssh = spawn(&command).expect(&format!("Unknown command: {:?}", command));
        loop {
            match expectrl::check!(
                &mut ssh,
                _ = "(yes/no/[fingerprint])" => {
                    ssh.send_line("yes").unwrap();
                },
                _ = "password:" => {
                    ssh.send_line("1").unwrap();
                    break;
                },
                default  => {
                    println!("unknown ssh output, will be exit.");
                    break;
                },
            ) {
                Err(expectrl::Error::Eof) => break,
                result => result.expect("Check output failed"),
            };
        }

        self.ssh = Some(ssh);

        Ok(())
    }
}

pub fn check_ssh_tunnels(ssh_tunnel_map: &mut HashMap<String, SSHTunnel>) {
    ssh_tunnel_map.iter_mut().for_each(|(k, v)| {
        if let Some(ref s) = v.ssh {
            if !s.is_alive() {
                v.start_tunnel().unwrap();
            }
        }
    });
}
