fn main() {
    // use std::env;
    // use std::path::PathBuf;
    // 以下代码告诉 Cargo ，一旦指定的文件 `src/register/register_view.fl` 发生了改变，就重新运行当前的构建脚本
    println!("cargo:rerun-if-changed=src/basic_view.fl");
    let g = fl2rust::Generator::default();
    //let out_path = PathBuf::from("src/register/register_view.rs");
    g.in_out("src/basic_view.fl", "src/basic_view.rs")
        .expect("Failed to generate rust from fl file!");
}
