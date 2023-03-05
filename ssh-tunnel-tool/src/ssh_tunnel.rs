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

lazy_static::lazy_static! {
    pub static ref RWLOCK_SSH_TUNNEL_MAP: RwLock<HashMap<String,SSHTunnel>> =
        RwLock::new(HashMap::<String,SSHTunnel>::new());
}

#[derive(Debug)]
pub struct SSHTunnel {
    id: String,
    name: String,
    forward_port: String,
    dst_host_port: String,
    ssh_user_server_port: String,
    cur_process: Option<JoinHandle<()>>,
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
            cur_process: Option::None,
        }
    }

    pub fn start_tunnel(&mut self) -> Result<()> {
        let mut cmd = Command::new("cmd");
        let args = [
            "/C",
            "ssh",
            "-NT",
            "-o",
            "ServerAliveInterval=60",
            "-o",
            "ServerAliveCountMax=3",
            "-L",
            &format!("{}:{}", self.forward_port, self.dst_host_port),
            &self.ssh_user_server_port,
        ];
        println!("cmd args:{:?}", args);
        let cmd = cmd.args(&args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

        let child = cmd.spawn().unwrap();
        let pid = child.id();

        let errt = thread::spawn(move || {
            let reader = BufReader::new(child.stderr.unwrap());
            for line in reader.lines() {
                println!("stderr:{:?}", line);
            }
        });

        //let t = thread::spawn(move || {
            let reader = BufReader::new(child.stdout.unwrap());
            let mut stdin = child.stdin.unwrap();
            //let mut stdin = BufWriter::new(&mut stdin);
            for line in reader.lines() {
                println!("stdout:{:?}", line);
                match line.unwrap().as_str() {
                    s if s.contains("(yes/no/[fingerprint])") => {
                        let eol: &str = "\n";
                        let input = "yes".to_string() + eol;
                        stdin.write(input.as_bytes()).unwrap();
                    }
                    s if s.contains("password:") => {
                        let eol: &str = "\n";
                        let input = "1".to_string() + eol;
                        stdin.write(input.as_bytes()).unwrap();
                        //write!(stdin, "{}", input).unwrap();
                        // let bytestring = input.as_bytes();
                        // stdin.write_all(bytestring).unwrap();
                    }
                    s => {
                        println!("pid:{pid}, ssh return:{s}");
                    }
                }
            }
        // });

        // self.cur_process = Some(t);

        Ok(())
    }
}

pub fn monitor_ssh_tunnels() {
    loop {
        match RWLOCK_SSH_TUNNEL_MAP.try_write() {
            Ok(ref mut ssh_tunnel_map) => {
                //let map = ssh_tunnel_map as HashMap<String, SSHTunnel>;
                ssh_tunnel_map.iter_mut().for_each(|(k, v)| {
                    if let Some(ref c) = v.cur_process {
                        if c.is_finished() {
                            v.start_tunnel().unwrap();
                        }
                    }
                });
            }
            Err(err) => panic!("get RWLOCK_SSH_TUNNEL_MAP error:{}", err),
        }

        thread::sleep(Duration::from_secs(10));
    }
}
