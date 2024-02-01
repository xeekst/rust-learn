// use fltk::{
//     app,
//     enums::FrameType,
//     frame::Frame,
//     image::{GifImage, PngImage},
//     prelude::*,
//     window::Window,
// };
// use std::{fs::File, path::Path, time::Duration};

// fn main() {
//     let app = app::App::default().with_scheme(app::Scheme::Gleam);
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

//     let mut fframe = Frame::default().with_size(360, 260).center_of(&wind);
//     fframe.set_frame(FrameType::EngravedBox);

//     wind.make_resizable(true);
//     wind.end();
//     wind.show();

//     let file = File::open("test.gif").unwrap();

//     let mut gif_opts = gif::DecodeOptions::new();
//     // Important:
//     gif_opts.set_color_output(gif::ColorOutput::Indexed);

//     let mut decoder = gif_opts.read_info(file).unwrap();
//     let mut screen = gif_dispose::Screen::new_decoder(&decoder);
//     let mut last_time = std::time::SystemTime::now();
//     // app::add_idle3(move |_| {
//     //     let cur_time = std::time::SystemTime::now();
//     //     let offset = cur_time.duration_since(last_time).unwrap();
//     //     if offset.as_millis() > 50 {
//     //         last_time = cur_time;
//     //         if let Some(frame) = decoder.read_next_frame().unwrap() {
//     //             screen.blit_frame(&frame).unwrap();

//     //             let frame_file = format!("{}.png", "base_name");
//     //             //println!("{}", frame_file);
//     //             let f = &frame_file.clone();
//     //             let b = lodepng::encode32(
//     //                 &screen.pixels.buf(),
//     //                 screen.pixels.width(),
//     //                 screen.pixels.height(),
//     //             )
//     //             .unwrap();

//     //             let mut image = PngImage::from_data(&b).unwrap();
//     //             image.scale(200, 200, true, true);
//     //             fframe.set_image(Some(image));
//     //             fframe.redraw();
//     //         }
//     //     }
//     // });
//     let interval = Duration::from_secs(1); // One second interval

//     app::add_timeout3(interval.as_secs() as f64, move |handler| {
//         if let Some(frame) = decoder.read_next_frame().unwrap() {
//             screen.blit_frame(&frame).unwrap();
//             let b = lodepng::encode32(
//                 &screen.pixels.buf(),
//                 screen.pixels.width(),
//                 screen.pixels.height(),
//             )
//             .unwrap();

//             let mut image = PngImage::from_data(&b).unwrap();
//             image.scale(200, 200, true, true);
//             fframe.set_image(Some(image));
//             fframe.redraw();
//         }
//         app::repeat_timeout3(0.1, handler);
//     });

//     app.run().unwrap();
// }


use fltk::{enums, prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    win.end();
    win.show();

    win.draw(|w| {
        use draw::*;
        // 白色窗口
        draw_rect_fill(0, w.h() / 2 - 10, w.w(), 20, enums::Color::Red);
        // // 画一个蓝色的圆
        // set_draw_color(enums::Color::Blue.inactive());
        // draw_pie(w.w() / 2 - 50, w.h() / 2 - 50, 100, 100, 0.0, 360.0);
        // // 让文字旋转一定角度
        // set_draw_color(enums::Color::Red);
        // set_font(enums::Font::Courier, 16);
        // draw_text_angled(45, "Hello World", w.w() / 2, w.h() / 2);
    });

    a.run().unwrap();
}
