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
use expectrl::{process::windows::WinProcess, spawn, Session};

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
    ssh: Option<
        Session<
            WinProcess,
            expectrl::stream::log::LoggedStream<
                expectrl::process::windows::ProcessStream,
                std::io::Stdout,
            >,
        >,
    >,
    status: Status,
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Started,
    Stoped,
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
        status: Status,
    ) -> SSHTunnel {
        SSHTunnel {
            id: id.to_string(),
            name: name.to_string(),
            forward_port,
            dst_host_port: dst_host_port.to_string(),
            ssh_username: ssh_username.to_string(),
            ssh_server: ssh_server.to_string(),
            ssh_port,
            ssh_pwd: ssh_pwd.to_string(),
            ssh: Option::None,
            forward_type: forward_type.to_string(),
            status,
        }
    }

    pub fn stop_tunnel(&mut self) {
        if let Some(ssh) = &self.ssh {
            self.status = Status::Stoped;
            drop(ssh);
            self.ssh = Option::None;
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

        let mut ssh = spawn(&command)
            .expect(&format!("Unknown command: {:?}", command))
            .with_log(std::io::stdout())?;

        let start_time = chrono::Local::now();

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
                default  => {
                    if (chrono::Local::now() - start_time) > chrono::Duration::seconds(3) {
                        eprintln!("timout 3s. will be break.");
                        break;
                    }
                }
            ) {
                Err(expectrl::Error::Eof) => break,
                result => result.expect("Check output failed"),
            };
        }

        println!("init ssh success.");
        self.ssh = Some(ssh);

        Ok(())
    }
}

pub fn check_ssh_tunnels(ssh_tunnel_map: &mut HashMap<usize, SSHTunnel>) {
    ssh_tunnel_map.iter_mut().for_each(|(k, v)| {
        if v.status == Status::Started {
            match v.ssh {
                Some(ref ssh) => {
                    if ssh.is_alive() {
                        return;
                    } else {
                        println!("ssh is deaded. will drop it and retry...");
                        drop(ssh);
                        v.start_tunnel().unwrap();
                    }
                }
                None => {
                    v.start_tunnel().unwrap();
                }
            }
        }
    });
}
