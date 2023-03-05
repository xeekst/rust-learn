
use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;



#[derive(Debug, Clone)]
pub struct BasicView {
    pub scroll_view: Scroll,
    pub tunnel_g1: Group,
    pub play_btn: Button,
    pub stop_btn: Button,
    pub menu: MenuBar,
}


impl BasicView {
    pub fn make_window() -> Self {
	let mut fl2rust_widget_0 = Window::new(575, 379, 800, 180, None);
	fl2rust_widget_0.end();
	fl2rust_widget_0.set_color(Color::by_index(54));
	fl2rust_widget_0.show();
	let mut scroll_view = Scroll::new(0, 20, 800, 160, None);
	scroll_view.end();
	scroll_view.set_color(Color::by_index(54));
	scroll_view.set_align(unsafe {std::mem::transmute(0)});
	fl2rust_widget_0.add(&scroll_view);
	let mut fl2rust_widget_1 = Group::new(0, 20, 786, 30, None);
	fl2rust_widget_1.end();
	fl2rust_widget_1.set_frame(FrameType::BorderBox);
	scroll_view.add(&fl2rust_widget_1);
	let mut fl2rust_widget_2 = Frame::new(40, 24, 50, 20, "Name");
	fl2rust_widget_1.add(&fl2rust_widget_2);
	let mut fl2rust_widget_3 = Frame::new(120, 25, 50, 20, "Type");
	fl2rust_widget_1.add(&fl2rust_widget_3);
	let mut fl2rust_widget_4 = Frame::new(185, 24, 85, 20, "Start/Stop");
	fl2rust_widget_1.add(&fl2rust_widget_4);
	let mut fl2rust_widget_5 = Frame::new(295, 24, 50, 20, "Forward port");
	fl2rust_widget_1.add(&fl2rust_widget_5);
	let mut fl2rust_widget_6 = Frame::new(400, 25, 50, 20, "Dst host:port");
	fl2rust_widget_1.add(&fl2rust_widget_6);
	let mut fl2rust_widget_7 = Frame::new(550, 25, 150, 20, "user@@sshhost:port");
	fl2rust_widget_1.add(&fl2rust_widget_7);
	let mut tunnel_g1 = Group::new(15, 50, 763, 35, None);
	tunnel_g1.end();
	tunnel_g1.set_frame(FrameType::BorderBox);
	tunnel_g1.set_color(Color::by_index(54));
	tunnel_g1.set_align(unsafe {std::mem::transmute(0)});
	scroll_view.add(&tunnel_g1);
	let mut play_btn = Button::new(197, 54, 24, 24, None);
	play_btn.set_image(Some(SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png")));
	play_btn.set_frame(FrameType::FlatBox);
	play_btn.set_color(Color::by_index(255));
	play_btn.set_align(unsafe {std::mem::transmute(16)});
	tunnel_g1.add(&play_btn);
	let mut stop_btn = Button::new(232, 54, 24, 24, None);
	stop_btn.set_image(Some(SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png")));
	stop_btn.set_frame(FrameType::FlatBox);
	stop_btn.set_color(Color::by_index(255));
	stop_btn.set_align(unsafe {std::mem::transmute(16)});
	stop_btn.deactivate();
	tunnel_g1.add(&stop_btn);
	let mut fl2rust_widget_8 = Input::new(22, 55, 95, 24, None);
	fl2rust_widget_8.set_label_type(LabelType::None);
	tunnel_g1.add(&fl2rust_widget_8);
	let mut fl2rust_widget_9 = Input::new(127, 56, 50, 24, None);
	fl2rust_widget_9.set_label_type(LabelType::None);
	tunnel_g1.add(&fl2rust_widget_9);
	let mut fl2rust_widget_10 = Input::new(292, 56, 50, 24, None);
	fl2rust_widget_10.set_label_type(LabelType::None);
	tunnel_g1.add(&fl2rust_widget_10);
	let mut fl2rust_widget_11 = Input::new(374, 56, 110, 24, None);
	fl2rust_widget_11.set_label_type(LabelType::None);
	tunnel_g1.add(&fl2rust_widget_11);
	let mut fl2rust_widget_12 = Input::new(507, 56, 228, 24, None);
	fl2rust_widget_12.set_label_type(LabelType::None);
	tunnel_g1.add(&fl2rust_widget_12);
	let mut menu = MenuBar::new(0, 0, 786, 20, None);
	menu.end();
	fl2rust_widget_0.add(&menu);
	menu.add("+", Shortcut::None, MenuFlag::Normal, |_| {});
	menu.add("about", Shortcut::None, MenuFlag::Normal, |_| {});
	Self { scroll_view, tunnel_g1, play_btn, stop_btn, menu, }
    }
}


