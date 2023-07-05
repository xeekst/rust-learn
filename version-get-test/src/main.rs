fn main() {
    // 从环境变量中获取版本号
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();

    println!(
        "Version: {}, 0.1.5.0 > {}  = {:?}",
        version,
        version,
        "0.1.5.0" > version.as_str()
    );

    for v in std::env::vars() {
        println!("env_var:{:?}", v);
    }
}
