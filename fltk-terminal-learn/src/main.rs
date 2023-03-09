use fltk::app;

use crate::view::UserInterface;

mod view;

fn main() {
    // not realy use full,, just display demo
    let app = app::App::default();
    let view = UserInterface::make_window();
    let name = "argag";
    println!("\x1b[93mError\x1b[0m");
    app.run().unwrap();
    println!("Hello, world!");
}
