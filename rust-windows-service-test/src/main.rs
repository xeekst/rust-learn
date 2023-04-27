#[macro_use]
extern crate windows_service;

use std::{ffi::OsString, time::Duration};
use windows_service::service_dispatcher;

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    // The entry point where execution will start on a background thread after a call to
    loop {
        info!("rust service print!!!!!!!!!!!!");
        std::thread::sleep(Duration::from_secs(3));
    }
}

fn main() -> Result<(), windows_service::Error> {
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    service_dispatcher::start("my_rust_service", ffi_service_main)?;
    Ok(())
}

use std::{env, fs, path::PathBuf};

use fern::colors::{Color, ColoredLevelConfig};
use log::info;

pub fn init_or_panic() {
    init_env();
    let init_r = init_log();
    if let Err(err) = init_r {
        let err_text = format!("init error: {err}");
        eprintln!("{err_text}");
        panic!("{err_text}");
    }

    info!("this is in {} os.", std::env::consts::OS)
}

fn init_env() {
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }
}

fn init_log() -> Result<(), Box<dyn std::error::Error>> {
    let log_filename = "agent_main";

    let logs_dir = PathBuf::from("logs");
    if !logs_dir.exists() {
        fs::create_dir(logs_dir)?;
    }

    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Green);
    fern::Dispatch::new()
        // write console
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}]{}[{}] {}",
                        colors.color(record.level()),
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        message
                    ))
                })
                .level(log::LevelFilter::Info)
                //.filter(|meta_data| meta_data.level() == log::LevelFilter::Info || meta_data.level() == log::LevelFilter::Warn)
                .chain(std::io::stdout()),
        )
        // write file
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}]{}[{}] {}",
                        record.level(),
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        message
                    ))
                })
                .chain(
                    fern::Dispatch::new()
                        .level(log::LevelFilter::Debug)
                        //.filter(|meta_data| meta_data.level() == log::LevelFilter::Debug || meta_data.level() == log::LevelFilter::Trace)
                        .level_for("hyper", log::LevelFilter::Info)
                        .level_for("agent_core::network::network_ftp", log::LevelFilter::Info)
                        .level_for("agent_core::network::network_http", log::LevelFilter::Debug)
                        .chain(fern::DateBased::new(
                            "logs/",
                            format!("%Y-%m-%d-{log_filename}.debug.log"),
                        )),
                )
                .chain(fern::Dispatch::new().level(log::LevelFilter::Info).chain(
                    fern::DateBased::new("logs/", format!("%Y-%m-%d-{log_filename}.info.log")),
                ))
                .chain(
                    fern::Dispatch::new()
                        .level(log::LevelFilter::Error)
                        .level_for("hyper", log::LevelFilter::Error)
                        .level_for("agent_core::network::network_http", log::LevelFilter::Error)
                        .chain(fern::DateBased::new(
                            "logs/",
                            format!("%Y-%m-%d-{log_filename}.error.log"),
                        )),
                ),
        )
        // Apply globally
        .apply()?;

    // and log using log crate macros!
    info!("init log config success!");

    Ok(())
}
