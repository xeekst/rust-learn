
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
    pub tunnel_row: Group,
    pub index_txt: Frame,
    pub forward_type_choice: MenuButton,
    pub start_btn: Button,
    pub stop_btn: Button,
    pub name_iuput: Input,
    pub forward_port_iuput: ValueInput,
    pub dst_server_port_input: Input,
    pub ssh_username_iuput: Input,
    pub ssh_server_ip_iuput: Input,
    pub ssh_port_iuput: ValueInput,
    pub pwd_input: SecretInput,
    pub check_box: CheckButton,
    pub menu: MenuBar,
}


impl BasicView {
    pub fn make_window() -> Self {
	let mut fl2rust_widget_0 = Window::new(537, 479, 960, 200, None);
	fl2rust_widget_0.end();
	fl2rust_widget_0.set_color(Color::by_index(54));
	fl2rust_widget_0.show();
	let mut scroll_view = Scroll::new(0, 20, 960, 180, None);
	scroll_view.end();
	scroll_view.set_color(Color::by_index(54));
	scroll_view.set_label_font(Font::by_index(1));
	scroll_view.set_align(unsafe {std::mem::transmute(0)});
	fl2rust_widget_0.add(&scroll_view);
	let mut fl2rust_widget_1 = Group::new(0, 20, 946, 30, None);
	fl2rust_widget_1.end();
	fl2rust_widget_1.set_frame(FrameType::BorderBox);
	scroll_view.add(&fl2rust_widget_1);
	let mut fl2rust_widget_2 = Frame::new(115, 24, 45, 20, "Name");
	fl2rust_widget_1.add(&fl2rust_widget_2);
	let mut fl2rust_widget_3 = Frame::new(200, 25, 40, 20, "Type");
	fl2rust_widget_1.add(&fl2rust_widget_3);
	let mut fl2rust_widget_4 = Frame::new(250, 25, 70, 20, "Start/Stop");
	fl2rust_widget_1.add(&fl2rust_widget_4);
	let mut fl2rust_widget_5 = Frame::new(320, 24, 80, 20, "Forward port");
	fl2rust_widget_1.add(&fl2rust_widget_5);
	let mut fl2rust_widget_6 = Frame::new(415, 25, 80, 20, "Dst host:port");
	fl2rust_widget_1.add(&fl2rust_widget_6);
	let mut fl2rust_widget_7 = Frame::new(810, 25, 65, 20, "password");
	fl2rust_widget_1.add(&fl2rust_widget_7);
	let mut fl2rust_widget_8 = Frame::new(528, 25, 260, 20, "username @@ssh_server_ip : ssh_port");
	fl2rust_widget_1.add(&fl2rust_widget_8);
	let mut fl2rust_widget_9 = Frame::new(47, 25, 30, 20, "No.");
	fl2rust_widget_1.add(&fl2rust_widget_9);
	let mut tunnel_row = Group::new(15, 54, 913, 36, None);
	tunnel_row.end();
	tunnel_row.set_frame(FrameType::BorderBox);
	tunnel_row.set_color(Color::by_index(54));
	tunnel_row.set_align(unsafe {std::mem::transmute(0)});
	scroll_view.add(&tunnel_row);
	let mut index_txt = Frame::new(50, 62, 20, 20, "1");
	index_txt.set_color(Color::by_index(46));
	index_txt.set_label_font(Font::by_index(1));
	index_txt.set_label_color(Color::by_index(229));
	tunnel_row.add(&index_txt);
	let mut forward_type_choice = MenuButton::new(190, 62, 60, 20, "menu");
	forward_type_choice.end();
	tunnel_row.add(&forward_type_choice);
	let mut start_btn = Button::new(260, 59, 24, 24, None);
	start_btn.set_image(Some(SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png")));
	start_btn.set_frame(FrameType::FlatBox);
	start_btn.set_color(Color::by_index(255));
	start_btn.set_align(unsafe {std::mem::transmute(16)});
	tunnel_row.add(&start_btn);
	let mut stop_btn = Button::new(285, 59, 24, 24, None);
	stop_btn.set_image(Some(SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png")));
	stop_btn.set_frame(FrameType::FlatBox);
	stop_btn.set_color(Color::by_index(255));
	stop_btn.set_align(unsafe {std::mem::transmute(16)});
	stop_btn.deactivate();
	tunnel_row.add(&stop_btn);
	let mut name_iuput = Input::new(90, 61, 95, 24, None);
	name_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&name_iuput);
	let mut forward_port_iuput = ValueInput::new(325, 61, 70, 24, None);
	forward_port_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&forward_port_iuput);
	let mut dst_server_port_input = Input::new(400, 61, 110, 24, None);
	dst_server_port_input.set_label_type(LabelType::None);
	tunnel_row.add(&dst_server_port_input);
	let mut ssh_username_iuput = Input::new(522, 61, 78, 24, None);
	ssh_username_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&ssh_username_iuput);
	let mut ssh_server_ip_iuput = Input::new(620, 61, 85, 24, None);
	ssh_server_ip_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&ssh_server_ip_iuput);
	let mut ssh_port_iuput = ValueInput::new(715, 61, 80, 24, None);
	ssh_port_iuput.set_label_type(LabelType::None);
	tunnel_row.add(&ssh_port_iuput);
	let mut pwd_input = SecretInput::new(805, 61, 85, 24, None);
	pwd_input.set_label_type(LabelType::None);
	tunnel_row.add(&pwd_input);
	let mut fl2rust_widget_10 = Frame::new(600, 62, 20, 20, "@@");
	tunnel_row.add(&fl2rust_widget_10);
	let mut fl2rust_widget_11 = Frame::new(705, 62, 10, 20, ":");
	fl2rust_widget_11.set_label_font(Font::by_index(1));
	tunnel_row.add(&fl2rust_widget_11);
	let mut check_box = CheckButton::new(20, 65, 15, 15, "button");
	check_box.set_down_frame(FrameType::DownBox);
	check_box.set_color(Color::by_index(229));
	check_box.set_selection_color(Color::by_index(228));
	check_box.set_label_type(LabelType::None);
	check_box.set_label_color(Color::by_index(229));
	tunnel_row.add(&check_box);
	let mut menu = MenuBar::new(0, 0, 960, 20, None);
	menu.end();
	fl2rust_widget_0.add(&menu);
	menu.add("+", Shortcut::None, MenuFlag::Normal, |_| {});
	menu.add("about", Shortcut::None, MenuFlag::Normal, |_| {});
	Self { scroll_view, tunnel_row, index_txt, forward_type_choice, start_btn, stop_btn, name_iuput, forward_port_iuput, dst_server_port_input, ssh_username_iuput, ssh_server_ip_iuput, ssh_port_iuput, pwd_input, check_box, menu, }
    }
}


