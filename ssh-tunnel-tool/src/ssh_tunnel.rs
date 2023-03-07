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
    forward_type: String,
    forward_port: i32,
    dst_host_port: String,
    ssh_username: String,
    ssh_server: String,
    ssh_port: i32,
    ssh_pwd: String,
    ssh: Option<Session>,
}

impl SSHTunnel {
    pub fn new(
        id: &str,
        name: &str,
        forward_type: &str,
        forward_port: i32,
        dst_host_port: &str,
        ssh_username: &str,
        ssh_server: &str,
        ssh_port: i32,
        ssh_pwd: &str,
    ) -> SSHTunnel {
        SSHTunnel {
            id: id.to_string(),
            name: name.to_string(),
            forward_port: forward_port,
            dst_host_port: dst_host_port.to_string(),
            ssh_username: ssh_username.to_string(),
            ssh_server: ssh_server.to_string(),
            ssh_port: ssh_port,
            ssh_pwd: ssh_pwd.to_string(),
            ssh: Option::None,
            forward_type: forward_type.to_string(),
        }
    }

    pub fn start_tunnel(&mut self) -> Result<()> {
        let type_flag = match &self.forward_type {
            s if s == "Local" => "-L",
            s if s == "Remote" => "-R",
            _ => "-L",
        };
        let command =
            format!(
            "ssh -N -o ServerAliveInterval=60 -o ServerAliveCountMax=3 {type_flag} {0}:{1} {2}@{3} -p {4}",
            self.forward_port, self.dst_host_port, self.ssh_username,self.ssh_server,self.ssh_port
        );
        println!("command:{}", command);

        let mut ssh = spawn(&command).expect(&format!("Unknown command: {:?}", command));
        loop {
            match expectrl::check!(
                &mut ssh,
                _ = "(yes/no/[fingerprint])" => {
                    println!("input yes");
                    ssh.send_line("yes").unwrap();
                },
                _ = "password:" => {
                    println!("input password");
                    ssh.send_line(&self.ssh_pwd).unwrap();
                    break;
                },
                // default  => {
                //     println!("default");
                //     let mut buf = String::from("");
                //     ssh.read_line(&mut buf).unwrap();
                //     eprintln!("unknown ssh output:{:#?}, will be exit.", buf);
                //     break;
                // },
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
