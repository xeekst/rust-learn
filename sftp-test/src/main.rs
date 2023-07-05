use std::{
    fs::{self, File},
    io::{self, Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
    time::Instant,
};

use ssh2::{Session, Sftp};

fn main() {
    let sftp = get_sftp();
    let file = sftp.realpath(Path::new("/")).unwrap();
    let stat = sftp.lstat(Path::new("/vincent/678.txt")).unwrap();

    println!("path:{:?}", file.as_path());
    println!("stat:{:?}", stat);

    traverse_dir();

    download_file(&sftp, "UnattendLog.zip", "/vincent/UnattendLog.zip").unwrap();

    upload_file(&sftp, "UnattendLog.zip", "/vincent/tmp.zip").unwrap();

    // 开始计时
    let start = Instant::now();
    println!(
        "get_file_count:{}",
        get_file_count(&sftp, "/atm-cloud/atm-tools/atm-agent/proc").unwrap()
    );

    download_dir(&sftp, "proc", "/atm-cloud/atm-tools/atm-agent/proc").unwrap();

    // 计算经过的时间
    let duration = start.elapsed();

    // 打印时间信息
    println!("执行耗时: {:?}", duration);

    upload_dir(&sftp, "proc", "/vincent/atm-agent2/proc2/22").unwrap();
}

fn get_sftp() -> Sftp {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("ip:port").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("user", "pwd").unwrap();
    let sftp = sess.sftp().unwrap();

    sftp
}

fn traverse_dir() {
    let sftp = get_sftp();
    // start read dir
    let es = sftp
        .readdir(Path::new("/atm-cloud/atm-tools/atm-agent/proc"))
        .unwrap();
    for e in es {
        println!("{:?}", e);
    }
    // end read dir
}

fn get_file_count(sftp: &Sftp, remote_path: &str) -> Result<i32, io::Error> {
    let entries = sftp.readdir(Path::new(remote_path))?;
    let mut file_count = 0;

    for (entry_path, entry_stat) in entries {
        let filename = entry_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let remote_entry_path = format!("{}/{}", remote_path, filename);

        if entry_stat.is_dir() {
            file_count += get_file_count(sftp, &remote_entry_path)?;
        } else {
            file_count += 1;
        }
    }

    Ok(file_count)
}

fn download_file(sftp: &Sftp, local_path: &str, remote_path: &str) -> Result<(), io::Error> {
    let mut remote_file = sftp.open(Path::new(remote_path)).unwrap();
    let mut local_file = File::create(local_path)?;
    io::copy(&mut remote_file, &mut local_file)?;

    Ok(())
}

fn upload_file(sftp: &Sftp, local_path: &str, remote_path: &str) -> Result<(), io::Error> {
    let mut local_file = File::open(local_path)?;
    let mut remote_file = sftp.create(Path::new(remote_path))?;
    io::copy(&mut local_file, &mut remote_file)?;

    Ok(())
}

fn download_dir(sftp: &Sftp, local_path: &str, remote_path: &str) -> Result<(), io::Error> {
    let entries = sftp.readdir(Path::new(remote_path))?;
    if !PathBuf::from(local_path).exists() {
        fs::create_dir_all(local_path)?;
    }

    for (entry_path, entry_stat) in entries {
        let filename = entry_path
            .file_name()
            .map_or(
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "file name is Invalid OsStr",
                )),
                |f| Ok(f),
            )?
            .to_string_lossy()
            .into_owned();
        let remote_entry_path = format!("{}/{}", remote_path, filename);
        let local_entry_path = format!("{}/{}", local_path, filename);

        if entry_stat.is_dir() {
            // 如果是子文件夹，递归下载
            fs::create_dir_all(&local_entry_path)?;
            download_dir(sftp, &local_entry_path, &remote_entry_path)?;
        } else {
            // 如果是文件，下载到本地
            let mut remote_file = sftp.open(Path::new(&remote_entry_path))?;
            let mut local_file = fs::File::create(&local_entry_path)?;
            io::copy(&mut remote_file, &mut local_file)?;
        }
    }

    Ok(())
}

fn upload_dir(sftp: &Sftp, local_path: &str, remote_path: &str) -> Result<(), io::Error> {
    if !sftp.stat(Path::new(remote_path)).is_ok() {
        println!("create_dir_if_not_exist:{:?}", remote_path);
        create_dir_if_not_exist(sftp, remote_path)?;
    }

    let entries = fs::read_dir(local_path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let remote_entry_path = format!("{}/{}", remote_path, entry_name);
        if entry_path.is_dir() {
            upload_dir(sftp, &entry_path.to_string_lossy(), &remote_entry_path)?;
        } else {
            upload_file(sftp, &entry_path.to_string_lossy(), &remote_entry_path)?;
        }
    }

    Ok(())
}

fn create_dir_if_not_exist(sftp: &Sftp, remote_path: &str) -> Result<(), io::Error> {
    if !remote_path.starts_with("/") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "remote path must be start with / ",
        ));
    }

    if !sftp.stat(Path::new(remote_path)).is_ok() {
        let remote_dirs: Vec<&str> = remote_path.split("/").collect();
        let parent_dir = remote_dirs[0..remote_dirs.len() - 1].join("/");
        if parent_dir.as_str() == "" {
            return Ok(());
        }
        create_dir_if_not_exist(sftp, parent_dir.as_str())?;
    } else {
        return Ok(());
    }

    sftp.mkdir(Path::new(remote_path), 0o755)?;

    Ok(())
}
