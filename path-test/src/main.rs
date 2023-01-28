use std::{fs, path::Path};

use pbr::ProgressBar;
use rand::prelude::*;
use std::thread;
use std::time::Duration;
use fern::*;
use log::{debug, info, trace, warn};
use chrono::*;


fn main() {
    test_log();
    test_bar();
    println!(
        "std::e&nv::current_dir():{}",
        std::env::current_dir().unwrap().to_str().unwrap()
    );

    //cargo 可能不一样
    let exe = std::env::current_exe().unwrap();
    println!("std::env::current_exe():{:#?}", exe.display());
    println!("is existed: {}", exe.exists());

    let exe_dir = exe.parent().unwrap();
    println!("exe_dir:{:?}", exe_dir.to_str());

    let paths = fs::read_dir(exe_dir.to_str().unwrap()).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    // for var in std::env::vars() {
    //     println!("{:?}", var);
    // }

    let path_join = Path::new("\\\\?\\D:\\diasdfffr1").join("dir2\\sAForge.Imaging.dll");

    println!("path_join:{:?}", path_join);

    let dir = std::env::current_exe().unwrap();
    let dir = dir.canonicalize().expect("the current exe should exist");
    let dir = dir.parent().expect("the current exe should be a file");
    let dir = dir.to_string_lossy().to_owned();

    println!("dir:{}", dir);
    let str = "-rw-r--r-- 1 ftp ftp          44546 Feb 15  2022 EULA - Microsoft  Device Configuration Validation Toolkit.docx";
    let st2 = "-rw-rw-rw- 1 ftp ftp       152382583 May 13 02:55 Process.rar";
    let vec = str.split("2022 ");

    let mut index = 0;
    for item in vec {
        println!("{index}:{}", item);
        index += 1;
    }

    println!("Hello, world!");
}

fn test_log() {
    //! With fern, we can:

    // Configure logger at runtime
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply().unwrap();

    // and log using log crate macros!
    info!("hello, world!");
}

fn test_bar() {
    let count = 5;
    let mut pb = ProgressBar::new(count);
    pb.format("╢▌▌░╟");
    for _ in 0..count {
        pb.inc();
        let n = thread_rng().gen_range(0..100);
        thread::sleep(Duration::from_millis(1));
    }
    pb.finish_println("done!");
}
