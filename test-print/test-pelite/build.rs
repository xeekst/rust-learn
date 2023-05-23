#[cfg(target_os = "windows")]
use winres;

#[cfg(target_os = "windows")]
fn main() {
    use std::io::Write;
    // only build the resource for release builds
    // as calling rc.exe might be slow
    let res = winres::WindowsResource::new();
    match res.compile() {
        Err(error) => {
            write!(std::io::stderr(), "{}", error).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
