use std::{
    fs::{self, File},
    io::Read,
};

use serde_derive::Deserialize;
use toml::Value;

#[derive(Debug, Deserialize)]
struct ConfigTest {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Debug, Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    server_01: Server,
}

#[derive(Debug, Deserialize)]
struct Server {
    ip: String,
    port: u32,
    user: String,
    password: String,
    exportports: Vec<u32>,
    sftp: Sftp,
}

#[derive(Debug, Deserialize)]
struct Sftp {
    user: String,
    password: String,
}

fn main() {
    let text = fs::read_to_string("test.toml").unwrap();
    let dict = text.parse::<Value>().unwrap();
    println!("dict:{:#?}", dict);

    let config_toml: Config = toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap();

    println!("config.toml:{:#?}", config_toml);

    let config: ConfigTest = toml::from_str(
        r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#,
    )
    .unwrap();

    println!("config:{:?}", config);
    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, None);
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
}
