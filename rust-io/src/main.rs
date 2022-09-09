use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    path::{self, Path, PathBuf},
};

use chrono::{DateTime, Utc};

fn main() {
    let path = path::PathBuf::from("simplefs.txt");

    // 创建文件
    // 写入文件
    fs::write(&path, "this is content").unwrap();
    // 读取文件
    let content = fs::read(&path).unwrap();
    println!("read file: {:?}", String::from_utf8(content));
    // 删除文件
    fs::remove_file(path).unwrap();

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

    let dirpath = PathBuf::from("dir1/dir2/dir3/dir4");
    // 创建目录
    fs::create_dir_all(&dirpath).unwrap();
    assert!(Path::is_dir(&dirpath));

    // 删除目录及子目录
    fs::remove_dir_all("dir1/dir2").unwrap();
    // 删除一个非空目录
    fs::remove_dir("dir1").unwrap();

    //Path & PathBuf
    let dirpath = Path::new("dir1");
    println!("dirpath: {:?}", dirpath);
    println!("dirpath buf: {:?}", dirpath.to_path_buf());
    let dirpath_join = dirpath.join("dir2").join("dir3");
    println!("dirpath_join: {:?}", dirpath_join);

    let dirpath_buf = PathBuf::from("dir1/dir2/dir3/dir4");
    println!("dirpath_buf: {:?}", dirpath_buf);
    let dirbufjoin = dirpath_buf.join("dir2").join("dir3");

    //File 对一个文件进行只读操作
    // 创建文件
    //let _ = File::create("stdfsFile").unwrap();
    // 只读方式
    let file = File::open("multifile").unwrap();
    println!("ready only file: {:?}", file.metadata());
    // 按行读大文件
    println!("Read a text file line by line");
    read_file_line_by_line("multifile").unwrap();
    // 按buff来读
    println!("\nRead a file with a buffer");
    read_file_buffer("multifile").unwrap();
    println!("Read a file all content");
    read_file_string("multifile").unwrap();

    // 写的文件
    let mut mutfile = OpenOptions::new()
        .create(true)
        .append(true)
        .open("appendfile")
        .unwrap();
    mutfile.write(b"append file success").unwrap();
    mutfile.flush().unwrap();

    //DirBuilder
    //fs::DirBuilder::
    fs::remove_dir("dirbuilder").unwrap();
    let dirbuilder = DirBuilder::new();
    let build_r = dirbuilder.create("dirbuilder").unwrap();

    println!("Hello, world!");
}

// 1. read large file line by line
fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

// 2. read large file by buff
fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 64;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;

    loop {
        let read_count = file.read(&mut buffer)?;
        println!("{:?}", &buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}

// 3. read small file by once
fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    println!("read_file_string: {:?}", data);
    Ok(data)
}
