use std::{path::Path, error::Error, ptr::null_mut};

use fs_extra::dir::CopyOptions;
use winapi::um::winver::{GetFileVersionInfoSizeW, GetFileVersionInfoW};

fn main() {
    let mut options = CopyOptions::new();
    //表明只copy 目录dir1中的内容 而不是把目录 dir1 全部copy 过去
    options.content_only = true;
    // 源码中是这样使用的
    // if !dir.exists() {
    // if options.copy_inside {
    //    create_all(dir, false)?;
    // } else {
    //  create(dir, false)?;
    // }
    //}
    //
    options.copy_inside = true;
    options.overwrite = true;
    //fs_extra::dir::copy("dir1", "dir_t23", &options).unwrap();
    
}
