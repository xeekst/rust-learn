use clap::{arg, command, Parser, Subcommand, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// ip address of ssh server
    #[arg(long)]
    pub host: String,

    /// port of ssh server
    #[arg(long)]
    pub port: u16,

    /// username of ssh server
    #[arg(short, long)]
    pub user: String,

    /// password of ssh server
    #[arg(long)]
    pub pwd: String,

    /// local listen port for accepting tcp request
    #[arg(short, long)]
    pub local_port: u16,

    /// remote host for ssh server, will be connect this address from ssh server to remote_host
    #[arg(long)]
    pub remote_host: String,

    /// remote port for remote_host
    #[arg(long)]
    pub remote_port: u16,
}
