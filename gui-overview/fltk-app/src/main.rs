use fltk::{prelude::*, *};
mod myuifile;

fn main() {
    let app = app::App::default();
    let mut ui = myuifile::UserInterface::make_window();
    let mut win = ui.my_win.clone();
    // ui.btn1.set_callback(move |b| {
    //     b.set_label("clicked");
    //     win.set_label("Button clicked");
    //     println!("Works!");
    // });
    win.end();
    win.show();
    app.run().unwrap();
}
