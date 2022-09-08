use std::{
    fs,
    path::{self, Path},
};

use chrono::{DateTime, Utc};

fn main() {
    let path = path::PathBuf::from("simplefs.txt");

    // 写入文件
    fs::write(&path, "this is content").unwrap();
    // 读取文件
    let content = fs::read(&path).unwrap();
    println!("read file: {:?}", String::from_utf8(content));

    // 读取目录
    let dir = fs::read_dir("./").unwrap();
    for e in dir {
        let e = e.unwrap();
        let meta = e.metadata().unwrap();
        let datetime: DateTime<Utc> = meta.created().unwrap().into();
        let create_time = datetime.format("%Y-%m-%d %T");
        println!(
            r#"name: {:?}, path: {:?}, is_dir: {:?}, is_file: {:?}, is read only: {:?}, Last modified: {:?} seconds, create: {:?}"#,
            e.file_name(),
            e.path(),
            meta.is_dir(),
            meta.is_file(),
            meta.permissions().readonly(),
            meta.modified().unwrap().elapsed().unwrap().as_secs(),
            create_time.to_string()
        );
    }

    println!("Hello, world!");
}
