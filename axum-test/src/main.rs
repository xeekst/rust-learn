use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 50300));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
// use std::io::{self, BufRead};
// use std::{
//     io::{BufReader, Error},
//     os::windows::process::CommandExt,
//     process::{Child, Command, Stdio},
//     thread,
// };

// use anyhow::{anyhow, Ok, Result};
// use std::sync::mpsc::{self, TryRecvError};
// use std::time::Duration;

// pub fn exec_windows_cmd_wait_output(cmd_with_args: &str) -> String {
//     let child = Command::new("cmd")
//         .arg("/c")
//         .raw_arg(format!("chcp 65001 2>NUL 1>NUL && {}", cmd_with_args))
//         .output()
//         .unwrap();
//     let buf = child.stdout;
//     let text = std::str::from_utf8(&buf).unwrap();

//     String::from(text)
// }

// const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;
// const WINDOWS_DETACHED_PROCESS: u32 = 0x00000008;
// const WINDOWS_CREATE_NEW_CONSOLE: u32 = 0x00000010;
// pub fn exec_windows_cmd_without_cmdc_nowait(
//     cmd: &str,
//     args: &str,
//     output_handler: fn(Result<String, Error>),
// ) -> Result<u32> {
//     let child = Command::new(cmd)
//         .raw_arg(args)
//         .creation_flags(WINDOWS_CREATE_NO_WINDOW)
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()?;
//     let id = &child.id();

//     let _ = handle_stdout_stderr(child, cmd, args, output_handler)?;

//     Ok(*id)
// }

// fn handle_stdout_stderr(
//     child: Child,
//     cmd: &str,
//     args: &str,
//     output_handler: fn(Result<String, Error>),
// ) -> Result<()> {
//     match child.stdout {
//         Some(child_out) => {
//             thread::spawn(move || {
//                 let reader = BufReader::new(child_out);
//                 for line in reader.lines() {
//                     output_handler(line)
//                 }
//                 // 非阻塞每次读取缓存区所有行
//                 //let mut line = String::new();
//                 // loop {
//                 //     reader.read_line(&mut line).unwrap();
//                 //     println!("{}", line);
//                 // }
//             });
//         }
//         None => return Err(anyhow!("{:?} {:?} child.stdout is None", cmd, args)),
//     }

//     match child.stderr {
//         Some(child_out) => {
//             thread::spawn(move || {
//                 let reader = BufReader::new(child_out);
//                 for line in reader.lines() {
//                     output_handler(line)
//                 }
//             });
//         }
//         None => return Err(anyhow!("{:?} {:?} child.stderr is None", cmd, args)),
//     }

//     Ok(())
// }

// fn main() {
//     // loop{
//     //     println!("Press enter to terminate the child thread");
//     //     thread::sleep(Duration::from_millis(500));
//     // }
//     // return;
//     println!("Press enter to terminate the child thread");
//     let (tx, rx) = mpsc::channel();

//     let t = thread::spawn(move || loop {
//         println!("Working...");
//         let r = exec_windows_cmd_without_cmdc_nowait(
//             r#"D:\loopgo.exe"#,
//             "",
//             |e| println!("out:{:?}", e.unwrap()),
//         );
//         thread::sleep(Duration::from_millis(3500));
//         if let Ok(process) = heim::process::get(pid).await {
//             process
//                 .kill()
//                 .await
//                 .map_err(|_| "Cannot kill process".to_string())?;
//             Ok(format!("Process with pid {} killed successfully!", pid))
//         } else {
//             Err(format!("Cannot get process with pid {}", pid))
//         }
//         panic!("normal exit");
//         // match rx.try_recv() {
//         //     Ok(_) | Err(TryRecvError::Disconnected) => {
//         //         println!("Terminating.");
//         //         break;
//         //     }
//         //     _ => (),
//         // }
//     });
//     loop {
//         thread::sleep(Duration::from_millis(500));
//         println!("is finish:{:?}", t.is_finished());
//         if t.is_finished() {
//             drop(t);
//             break;
//         }
//     }
//     let mut line = String::new();
//     let stdin = io::stdin();
//     let _ = stdin.lock().read_line(&mut line);

//     let _ = tx.send(());
// }
