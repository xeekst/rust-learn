
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
    pub remote_host_input: Input,
    pub forward_port_iuput: IntInput,
    pub remote_port_input: Input,
    pub ssh_username_iuput: Input,
    pub ssh_server_ip_iuput: Input,
    pub ssh_port_iuput: IntInput,
    pub pwd_input: SecretInput,
    pub menu: MenuBar,
}


impl BasicView {
    pub fn make_window() -> Self {
	let mut main_window = Window::new(416, 217, 960, 300, None);
	main_window.end();
	main_window.set_color(Color::by_index(54));
	main_window.size_range(960, 200, 960, 900);
	main_window.show();
	let mut scroll_view = Scroll::new(0, 50, 960, 250, None);
	scroll_view.end();
	scroll_view.set_color(Color::by_index(54));
	scroll_view.set_label_font(Font::by_index(1));
	scroll_view.set_align(unsafe {std::mem::transmute(0)});
	main_window.add(&scroll_view);
	let mut tunnel_row = Group::new(15, 54, 913, 121, "g_id");
	tunnel_row.end();
	tunnel_row.set_frame(FrameType::BorderBox);
	tunnel_row.set_color(Color::by_index(54));
	tunnel_row.set_label_color(Color::by_index(54));
	tunnel_row.set_align(unsafe {std::mem::transmute(0)});
	scroll_view.add(&tunnel_row);
	let mut fl2rust_widget_0 = Frame::new(15, 93, 913, 3, None);
	fl2rust_widget_0.set_frame(FrameType::FlatBox);
	fl2rust_widget_0.set_color(Color::by_index(38));
	fl2rust_widget_0.set_label_type(LabelType::None);
	tunnel_row.add(&fl2rust_widget_0);
	let mut forward_type_choice = Choice::new(585, 60, 75, 24, None);
	forward_type_choice.end();
	forward_type_choice.set_down_frame(FrameType::BorderBox);
	tunnel_row.add(&forward_type_choice);
	let mut index_txt = Frame::new(28, 62, 20, 20, "1");
	index_txt.set_color(Color::by_index(46));
	index_txt.set_label_font(Font::by_index(1));
	index_txt.set_label_color(Color::by_index(229));
	tunnel_row.add(&index_txt);
	let mut start_btn = Button::new(795, 61, 24, 24, None);
	start_btn.set_image(Some(SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png")));
	start_btn.set_frame(FrameType::FlatBox);
	start_btn.set_color(Color::by_index(255));
	start_btn.set_align(unsafe {std::mem::transmute(16)});
	tunnel_row.add(&start_btn);
	let mut stop_btn = Button::new(835, 61, 24, 24, None);
	stop_btn.set_image(Some(SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png")));
	stop_btn.set_frame(FrameType::FlatBox);
	stop_btn.set_color(Color::by_index(255));
	stop_btn.set_align(unsafe {std::mem::transmute(16)});
	stop_btn.deactivate();
	tunnel_row.add(&stop_btn);
	let mut del_btn = Button::new(875, 61, 24, 24, None);
	del_btn.set_image(Some(SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png")));
	del_btn.set_frame(FrameType::FlatBox);
	del_btn.set_color(Color::by_index(255));
	del_btn.set_align(unsafe {std::mem::transmute(16)});
	del_btn.deactivate();
	tunnel_row.add(&del_btn);
	let mut name_iuput = Input::new(55, 60, 480, 24, None);
	name_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&name_iuput);
	let mut fl2rust_widget_1 = Frame::new(155, 125, 145, 35, "local listen addr");
	fl2rust_widget_1.set_frame(FrameType::UpBox);
	fl2rust_widget_1.set_label_type(LabelType::Embossed);
	fl2rust_widget_1.set_label_color(Color::by_index(229));
	fl2rust_widget_1.set_align(unsafe {std::mem::transmute(1)});
	tunnel_row.add(&fl2rust_widget_1);
	let mut fl2rust_widget_2 = Frame::new(710, 125, 190, 35, "remote addr");
	fl2rust_widget_2.set_frame(FrameType::UpBox);
	fl2rust_widget_2.set_label_type(LabelType::Embossed);
	fl2rust_widget_2.set_label_color(Color::by_index(229));
	fl2rust_widget_2.set_align(unsafe {std::mem::transmute(1)});
	tunnel_row.add(&fl2rust_widget_2);
	let mut remote_host_input = Input::new(718, 131, 110, 24, None);
	remote_host_input.set_label_type(LabelType::None);
	tunnel_row.add(&remote_host_input);
	let mut fl2rust_widget_3 = Frame::new(50, 120, 40, 40, "💻");
	fl2rust_widget_3.set_label_size(40);
	tunnel_row.add(&fl2rust_widget_3);
	let mut forward_port_iuput = IntInput::new(215, 131, 70, 24, "0.0.0.0:");
	tunnel_row.add(&forward_port_iuput);
	let mut remote_port_input = Input::new(840, 131, 50, 24, ":");
	tunnel_row.add(&remote_port_input);
	let mut fl2rust_widget_4 = Frame::new(360, 113, 290, 60, "SSH Server");
	fl2rust_widget_4.set_frame(FrameType::UpBox);
	fl2rust_widget_4.set_label_type(LabelType::Embossed);
	fl2rust_widget_4.set_label_color(Color::by_index(229));
	fl2rust_widget_4.set_align(unsafe {std::mem::transmute(1)});
	tunnel_row.add(&fl2rust_widget_4);
	let mut ssh_username_iuput = Input::new(410, 145, 100, 24, "user:");
	tunnel_row.add(&ssh_username_iuput);
	let mut ssh_server_ip_iuput = Input::new(410, 117, 100, 24, "host:");
	tunnel_row.add(&ssh_server_ip_iuput);
	let mut ssh_port_iuput = IntInput::new(550, 117, 85, 24, "port:");
	tunnel_row.add(&ssh_port_iuput);
	let mut pwd_input = SecretInput::new(550, 145, 85, 24, "pwd:");
	tunnel_row.add(&pwd_input);
	let mut menu = MenuBar::new(0, 0, 960, 20, None);
	menu.end();
	menu.set_color(Color::by_index(46));
	main_window.add(&menu);
	menu.add("+", Shortcut::None, MenuFlag::Normal, |_| {});
	menu.add("about", Shortcut::None, MenuFlag::Normal, |_| {});
	let mut fl2rust_widget_6 = Group::new(0, 20, 944, 30, None);
	fl2rust_widget_6.end();
	fl2rust_widget_6.set_frame(FrameType::UpBox);
	fl2rust_widget_6.set_color(Color::by_index(46));
	main_window.add(&fl2rust_widget_6);
	let mut fl2rust_widget_7 = Frame::new(75, 25, 45, 20, "Name");
	fl2rust_widget_6.add(&fl2rust_widget_7);
	let mut fl2rust_widget_8 = Frame::new(600, 25, 40, 20, "Type");
	fl2rust_widget_6.add(&fl2rust_widget_8);
	let mut fl2rust_widget_9 = Frame::new(785, 25, 40, 20, "Start");
	fl2rust_widget_6.add(&fl2rust_widget_9);
	let mut fl2rust_widget_10 = Frame::new(825, 25, 40, 20, "Stop");
	fl2rust_widget_6.add(&fl2rust_widget_10);
	let mut fl2rust_widget_11 = Frame::new(25, 25, 30, 20, "No.");
	fl2rust_widget_6.add(&fl2rust_widget_11);
	let mut fl2rust_widget_12 = Frame::new(872, 25, 28, 20, "Del");
	fl2rust_widget_6.add(&fl2rust_widget_12);
	Self { main_window, scroll_view, tunnel_row, forward_type_choice, index_txt, start_btn, stop_btn, del_btn, name_iuput, remote_host_input, forward_port_iuput, remote_port_input, ssh_username_iuput, ssh_server_ip_iuput, ssh_port_iuput, pwd_input, menu, }
    }
}


