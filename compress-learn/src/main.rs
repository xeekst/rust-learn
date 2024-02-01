use anyhow::anyhow;
use log::error;
use zip::{ZipWriter, write::FileOptions, CompressionMethod};
use std::{fs::{self, File}, io::prelude::*, path::Path};

fn main() {
    let mut t = Path::new("UNZIP");
    //extract(Path::new("64.zip.001"), t).unwrap();
    unzip(&Path::new("x.zip"), &t, false).unwrap();
}

///解压
/// test.zip文件解压到d:/test文件夹下
///
fn extract(test: &Path, target: &Path) -> anyhow::Result<()> {
    let zipfile = std::fs::File::open(&test).unwrap();
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    if !target.exists() {
        fs::create_dir_all(target)?
    }
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.is_dir() {
            let target = target.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target)?;
        } else {
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = if !file_path.exists() {
                fs::File::create(file_path)?
            } else {
                fs::File::open(file_path)?
            };
            std::io::copy(&mut file, &mut target_file)?;
        }
    }

    Ok(())
}

pub fn unzip(source: &Path, target: &Path, is_ignore_err: bool) -> anyhow::Result<()> {
    let zipfile = std::fs::File::open(&source).unwrap();
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();
    zip.extract(target)?;

    // if !target.exists() {
    //     fs::create_dir_all(target)?
    // }
    // for i in 0..zip.len() {
    //     let mut file = zip.by_index(i)?;
    //     if file.is_dir() {
    //         let target = target.join(Path::new(&file.name().replace("\\", "")));
    //         fs::create_dir_all(target)?;
    //     } else {
    //         let file_path = target.join(Path::new(file.name()));
    //         if file_path.exists() {
    //             if let Err(err) = fs::remove_file(&file_path) {
    //                 if is_ignore_err {
    //                     error!(
    //                         "fs::remove_file target_file:{:?} error:{:?}",
    //                         file_path, err
    //                     );
    //                 } else {
    //                     return Err(anyhow!(
    //                         "fs::remove_file target_file:{:?} error:{:?}",
    //                         file_path,
    //                         err
    //                     ));
    //                 }
    //             }
    //         }
    //         let mut target_file = if !file_path.exists() {
    //             fs::File::create(file_path)?
    //         } else {
    //             fs::File::open(file_path)?
    //         };
    //         if let Err(err) = std::io::copy(&mut file, &mut target_file) {
    //             if is_ignore_err {
    //                 error!(
    //                     "unzip copy file from:{:?} to target_file:{:?} error:{:?}",
    //                     file.name(),
    //                     target_file,
    //                     err
    //                 );
    //             } else {
    //                 return Err(anyhow!(
    //                     "unzip copy file from:{:?} to target_file:{:?} error:{:?}",
    //                     file.name(),
    //                     target_file,
    //                     err
    //                 ));
    //             }
    //         }
    //     }
    // }

    Ok(())
}
