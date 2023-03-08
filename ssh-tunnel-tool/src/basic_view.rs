
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
    pub main_window: Window,
    pub scroll_view: Scroll,
    pub tunnel_row: Group,
    pub forward_type_choice: Choice,
    pub index_txt: Frame,
    pub start_btn: Button,
    pub stop_btn: Button,
    pub del_btn: Button,
    pub name_iuput: Input,
    pub forward_port_iuput: IntInput,
    pub dst_server_port_input: Input,
    pub ssh_username_iuput: Input,
    pub ssh_server_ip_iuput: Input,
    pub ssh_port_iuput: IntInput,
    pub pwd_input: SecretInput,
    pub at_box: Frame,
    pub box2: Frame,
    pub menu: MenuBar,
}


impl BasicView {
    pub fn make_window() -> Self {
	let mut main_window = Window::new(375, 350, 960, 200, None);
	main_window.end();
	main_window.set_color(Color::by_index(54));
	main_window.size_range(960, 200, 960, 800);
	main_window.show();
	let mut scroll_view = Scroll::new(0, 20, 960, 180, None);
	scroll_view.end();
	scroll_view.set_color(Color::by_index(54));
	scroll_view.set_label_font(Font::by_index(1));
	scroll_view.set_align(unsafe {std::mem::transmute(0)});
	main_window.add(&scroll_view);
	let mut fl2rust_widget_0 = Group::new(0, 20, 944, 30, None);
	fl2rust_widget_0.end();
	fl2rust_widget_0.set_frame(FrameType::BorderBox);
	fl2rust_widget_0.set_color(Color::by_index(46));
	scroll_view.add(&fl2rust_widget_0);
	let mut fl2rust_widget_1 = Frame::new(80, 24, 45, 20, "Name");
	fl2rust_widget_0.add(&fl2rust_widget_1);
	let mut fl2rust_widget_2 = Frame::new(180, 25, 40, 20, "Type");
	fl2rust_widget_0.add(&fl2rust_widget_2);
	let mut fl2rust_widget_3 = Frame::new(245, 25, 70, 20, "Start/Stop");
	fl2rust_widget_0.add(&fl2rust_widget_3);
	let mut fl2rust_widget_4 = Frame::new(315, 24, 80, 20, "Forward port");
	fl2rust_widget_0.add(&fl2rust_widget_4);
	let mut fl2rust_widget_5 = Frame::new(410, 25, 80, 20, "Dst host:port");
	fl2rust_widget_0.add(&fl2rust_widget_5);
	let mut fl2rust_widget_6 = Frame::new(805, 25, 65, 20, "password");
	fl2rust_widget_0.add(&fl2rust_widget_6);
	let mut fl2rust_widget_7 = Frame::new(523, 25, 260, 20, "username @@ssh_server_ip : ssh_port");
	fl2rust_widget_0.add(&fl2rust_widget_7);
	let mut fl2rust_widget_8 = Frame::new(25, 25, 30, 20, "No.");
	fl2rust_widget_0.add(&fl2rust_widget_8);
	let mut fl2rust_widget_9 = Frame::new(890, 25, 28, 20, "del");
	fl2rust_widget_0.add(&fl2rust_widget_9);
	let mut tunnel_row = Group::new(15, 54, 913, 36, "g_id");
	tunnel_row.end();
	tunnel_row.set_frame(FrameType::BorderBox);
	tunnel_row.set_color(Color::by_index(54));
	tunnel_row.set_label_color(Color::by_index(54));
	tunnel_row.set_align(unsafe {std::mem::transmute(0)});
	scroll_view.add(&tunnel_row);
	let mut forward_type_choice = Choice::new(165, 60, 75, 24, None);
	forward_type_choice.end();
	forward_type_choice.set_down_frame(FrameType::BorderBox);
	tunnel_row.add(&forward_type_choice);
	let mut index_txt = Frame::new(28, 62, 20, 20, "1");
	index_txt.set_color(Color::by_index(46));
	index_txt.set_label_font(Font::by_index(1));
	index_txt.set_label_color(Color::by_index(229));
	tunnel_row.add(&index_txt);
	let mut start_btn = Button::new(255, 59, 24, 24, None);
	start_btn.set_image(Some(SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png")));
	start_btn.set_frame(FrameType::FlatBox);
	start_btn.set_color(Color::by_index(255));
	start_btn.set_align(unsafe {std::mem::transmute(16)});
	tunnel_row.add(&start_btn);
	let mut stop_btn = Button::new(280, 59, 24, 24, None);
	stop_btn.set_image(Some(SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png")));
	stop_btn.set_frame(FrameType::FlatBox);
	stop_btn.set_color(Color::by_index(255));
	stop_btn.set_align(unsafe {std::mem::transmute(16)});
	stop_btn.deactivate();
	tunnel_row.add(&stop_btn);
	let mut del_btn = Button::new(890, 60, 24, 24, None);
	del_btn.set_image(Some(SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png")));
	del_btn.set_frame(FrameType::FlatBox);
	del_btn.set_color(Color::by_index(255));
	del_btn.set_align(unsafe {std::mem::transmute(16)});
	del_btn.deactivate();
	tunnel_row.add(&del_btn);
	let mut name_iuput = Input::new(55, 60, 95, 24, None);
	name_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&name_iuput);
	let mut forward_port_iuput = IntInput::new(320, 60, 70, 24, None);
	forward_port_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&forward_port_iuput);
	let mut dst_server_port_input = Input::new(395, 60, 110, 24, None);
	dst_server_port_input.set_label_type(LabelType::None);
	tunnel_row.add(&dst_server_port_input);
	let mut ssh_username_iuput = Input::new(517, 60, 78, 24, None);
	ssh_username_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&ssh_username_iuput);
	let mut ssh_server_ip_iuput = Input::new(615, 60, 85, 24, None);
	ssh_server_ip_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&ssh_server_ip_iuput);
	let mut ssh_port_iuput = IntInput::new(710, 60, 80, 24, None);
	ssh_port_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&ssh_port_iuput);
	let mut pwd_input = SecretInput::new(800, 60, 85, 24, None);
	pwd_input.set_label_type(LabelType::None);
	tunnel_row.add(&pwd_input);
	let mut at_box = Frame::new(595, 62, 20, 20, "@@");
	tunnel_row.add(&at_box);
	let mut box2 = Frame::new(700, 62, 10, 20, ":");
	box2.set_label_font(Font::by_index(1));
	tunnel_row.add(&box2);
	let mut menu = MenuBar::new(0, 0, 960, 20, None);
	menu.end();
	menu.set_color(Color::by_index(46));
	main_window.add(&menu);
	menu.add("+", Shortcut::None, MenuFlag::Normal, |_| {});
	menu.add("about", Shortcut::None, MenuFlag::Normal, |_| {});
	Self { main_window, scroll_view, tunnel_row, forward_type_choice, index_txt, start_btn, stop_btn, del_btn, name_iuput, forward_port_iuput, dst_server_port_input, ssh_username_iuput, ssh_server_ip_iuput, ssh_port_iuput, pwd_input, at_box, box2, menu, }
    }
}


