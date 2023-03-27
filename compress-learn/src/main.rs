use std::{fs, io::prelude::*, path::Path};

fn main() {
    let mut t = Path::new("HEC");
    extract(Path::new("HEC-platform.zip"), t).unwrap();
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
