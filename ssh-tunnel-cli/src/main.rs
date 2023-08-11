mod cli;
use std::{
    fmt::Debug,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, warn};
use ssh2::{Channel, Session};

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const BUFFER_SIZE: usize = 409600000;

fn main() {
    let cli = cli::Cli::parse();
    init_log().unwrap();
    let exit_signal = Arc::new(AtomicBool::new(false));
    let tx = Arc::clone(&exit_signal);
    ctrlc::set_handler(move || {
        tx.store(true, Ordering::SeqCst);
        TcpStream::connect(SocketAddr::new(LOCALHOST, cli.local_port)).unwrap();
        info!("Received Ctrl-C, exiting");
    })
    .expect("Error setting Ctrl-C handler");

    let ssh_session = new_session(&cli.host, cli.port, &cli.user, &cli.pwd).unwrap();
    listen_on_local_forwarded_port(
        Arc::new(ssh_session),
        Arc::clone(&exit_signal),
        cli.local_port,
        &cli.remote_host,
        cli.remote_port,
    )
    .unwrap();
}

fn new_session(ssh_host: &str, ssh_port: u16, username: &str, pwd: &str) -> Result<Session> {
    let connest_str = format!("{ssh_host}:{ssh_port}");
    let tcp = TcpStream::connect(&connest_str)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_password(username, pwd)?;

    if !session.authenticated() {
        return Err(anyhow!("connect to :{connest_str} , auth failed"));
    }

    session.set_keepalive(true, 10);

    Ok(session)
}

fn listen_on_local_forwarded_port(
    ssh_session: Arc<Session>,
    should_exit: Arc<AtomicBool>,
    local_port: u16,
    remote_host: &str,
    remote_port: u16,
) -> Result<()> {
    match TcpListener::bind((LOCALHOST, local_port)) {
        Ok(listener) => {
            info!("Listening on {LOCALHOST}:{local_port}");
            for stream in listener.incoming() {
                let cloned_session = Arc::clone(&ssh_session);
                let cloned_remote_host = remote_host.to_string().clone();
                if should_exit.load(std::sync::atomic::Ordering::SeqCst) {
                    warn!("Received close connection signal");
                    break;
                }

                match stream {
                    Ok(stream) => {
                        std::thread::spawn(move || {
                            handle_request(cloned_session, stream, &cloned_remote_host, remote_port)
                        });
                    }
                    Err(err) => return Err(anyhow::anyhow!("tcpstream incoming error:{:?}", err)),
                }
            }
        }
        Err(err) => {
            return Err(anyhow::anyhow!(
                "TcpListener bind {LOCALHOST}:{local_port} error:{:?}",
                err
            ))
        }
    }

    warn!("TCP Listener stopped");

    Ok(())
}

fn handle_request(
    ssh_session: Arc<Session>,
    mut stream: TcpStream,
    remote_host: &str,
    remote_port: u16,
) {
    match ssh_session.channel_direct_tcpip(remote_host, remote_port, None) {
        Ok(mut channel) => {
            //read tcp data from user TcpStream
            let req_buf = read_stream(&mut stream);

            info!("REQUEST ({} bytes)", req_buf.len());
            debug!(
                "REQUEST ({} bytes):{}",
                req_buf.len(),
                String::from_utf8_lossy(&req_buf)
            );

            //send the incoming request data over the channel to the remote_host:remote_port
            if let Err(e) = channel.write_all(&req_buf) {
                error!("Failed to forward request, error: {}", e);
                return;
            }

            channel.flush().unwrap();
            info!("channel flush");
            //read the response from the channel to the remote server
            let response_buf = read_channel(&mut channel);

            info!("stream write_all start");
            // forward the response to user request TcpStream
            if let Err(e) = stream.write_all(&response_buf[..]) {
                error!("Failed to write response, error:{}", e);
                return;
            };
            info!("stream write_all end");
            info!("stream flush start");
            stream.flush().unwrap();
            info!("stream flush end");

            info!("SENT {} BYTES AS RESPONSE", response_buf.len());
            debug!(
                "SENT {} BYTES AS RESPONSE:{}\n",
                response_buf.len(),
                String::from_utf8_lossy(&response_buf)
            );
            channel.close().expect("Failed to close channel");
        }
        Err(err) => error!("create channel:{remote_host}:{remote_port} error:{:?}", err),
    }
}

fn read_stream<R: Read + Debug>(mut stream: R) -> Vec<u8> {
    let mut request_full_buffer = vec![];
    loop {
        let mut buffer = vec![0; BUFFER_SIZE];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if !read_buf_bytes(&mut request_full_buffer, n, buffer) {
                    break;
                }
            }
            Err(e) => {
                error!("Error in reading request data: {:?}", e);
                break;
            }
        }
    }

    request_full_buffer
}

fn read_channel(channel: &mut Channel) -> Vec<u8> {
    let mut response_full_buffer = vec![];
    loop {
        let mut buffer = vec![0; BUFFER_SIZE];
        let future_stream = channel.read(&mut buffer);
    
        match future_stream {
            Ok(n) => {
                info!("channel:eof:{} read size:{n}", channel.eof());
                if !read_buf_bytes(&mut response_full_buffer, n, buffer) {
                    break;
                }
            }
            Err(e) => {
                error!("Error in reading request data: {:?}", e);
                break;
            }
        }
    }

    response_full_buffer
}

fn read_buf_bytes(full_req_buf: &mut Vec<u8>, reader_buf_len: usize, reader_buf: Vec<u8>) -> bool {
    if reader_buf_len == 0 {
        false
    } else {
        full_req_buf.append(&mut reader_buf[..reader_buf_len].to_vec());
        if reader_buf_len < BUFFER_SIZE {
            false
        } else {
            true
        }
    }
}

fn init_log() -> Result<()> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Green);
    fern::Dispatch::new()
        // write console
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}]{}[{}:{:?}] {}",
                        colors.color(record.level()),
                        chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                        //record.target(),
                        record.file().unwrap_or("<unnamed>"),
                        record.line().unwrap_or(0),
                        message
                            .to_string()
                            .split("\n\nStack backtrace:")
                            .collect::<Vec<&str>>()
                            .get(0)
                            .unwrap_or(&"")
                    ))
                })
                .level(log::LevelFilter::Info)
                //.filter(|meta_data| meta_data.level() == log::LevelFilter::Info || meta_data.level() == log::LevelFilter::Warn)
                .chain(std::io::stdout()),
        )
        // Apply globally
        .apply()?;

    // and log using log crate macros!
    info!("init log config success!");

    Ok(())
}
