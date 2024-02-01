use fltk::{enums::*, prelude::*, *};

fn move_button(mut btn: button::Button, handle: app::TimeoutHandle) {
    btn.set_size(btn.w() - 2, btn.h());
    btn.parent().unwrap().redraw();
    if btn.w() == 20 {
        app::remove_timeout3(handle);
    } else {
        app::repeat_timeout3(0.016, handle);
    }
}

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    win.set_color(Color::White);
    let mut btn = button::Button::new(0, 0, 100, 300, None);
    style_btn(&mut btn);
    btn.clear_visible_focus();
    win.end();
    win.show();

    btn.set_callback(|b| {
        let btn = b.clone();
        app::add_timeout3(0.016, move |handle| {
            let btn = btn.clone();
            move_button(btn, handle)
        });
    });

    a.run().unwrap();
}

fn style_btn(btn: &mut button::Button) {
    btn.set_color(Color::from_hex(0x42A5F5));
    btn.set_selection_color(Color::from_hex(0x42A5F5));
    btn.set_frame(FrameType::FlatBox);
}
