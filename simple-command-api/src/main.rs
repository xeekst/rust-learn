use std::env;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use fern::colors::{Color, ColoredLevelConfig};
use log::info;
use tokio;

mod encoding_config;
mod system_process;

#[tokio::main]
async fn main() {
    init_log().unwrap();
    let args: Vec<String> = env::args().collect();
    info!("args:{:?}", args);
    let port = args[1].parse::<u16>().unwrap();

    let app = Router::new()
        .route("/", get(is_alive))
        .route("/restart_sshd", get(restart_sshd));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn is_alive() -> &'static str {
    "Yes, I am alive..."
}

async fn restart_sshd() -> impl IntoResponse {
    let cmd = "net stop sshd && net start sshd";

    info!("restart_sshd cmd:{cmd}");
    match system_process::exec_cmd_nowindow_wait_output(cmd) {
        Ok(r) => (
            StatusCode::OK,
            Json(format!("success restart! out:{:?}", r)),
        ),
        Err(err) => (
            StatusCode::OK,
            Json(format!("failed restart_sshd:{:?}", err)),
        ),
    }
}

fn init_log() -> Result<(), Box<dyn std::error::Error>> {
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
                        .level_for("hyper", log::LevelFilter::Info)
                        .chain(fern::log_file("debug.log")?),
                )
                .chain(
                    fern::Dispatch::new()
                        .level(log::LevelFilter::Info)
                        .chain(fern::log_file("info.log")?),
                )
                .chain(
                    fern::Dispatch::new()
                        .level(log::LevelFilter::Error)
                        .level_for("hyper", log::LevelFilter::Error)
                        .chain(fern::log_file("error.log")?),
                ),
        )
        .apply()?;

    info!("init log config success!");

    Ok(())
}
