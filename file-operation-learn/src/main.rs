use fs_extra::dir::CopyOptions;

fn main() {
    let mut options = CopyOptions::new();
    //表明只copy 目录dir1中的内容 而不是把目录 dir1 全部copy 过去
    options.content_only = true;
    options.copy_inside = true;
    options.overwrite = true;
    fs_extra::dir::copy("dir1", "dir_t", &options).unwrap();

    println!("Hello, world!");
   
}
