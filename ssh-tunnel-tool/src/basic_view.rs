
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
    pub local_tunnel_group: Group,
    pub gray_line: Frame,
    pub forward_type_choice: Choice,
    pub index_txt: Frame,
    pub start_btn: Button,
    pub stop_btn: Button,
    pub del_btn: Button,
    pub name_iuput: Input,
    pub local_listen_addr_box: Frame,
    pub remote_addr_box: Frame,
    pub remote_host_input: Input,
    pub img_pc: Frame,
    pub img_right_arrow1: Frame,
    pub img_right_arrow2: Frame,
    pub img_right_arrow3: Frame,
    pub forward_port_input: IntInput,
    pub remote_port_input: Input,
    pub ssh_server_box: Frame,
    pub img_firewall: Frame,
    pub ssh_username_input: Input,
    pub ssh_server_ip_input: Input,
    pub ssh_port_input: IntInput,
    pub ssh_pwd_input: SecretInput,
    pub remote_tunnel_group: Group,
    pub forward_type_choice_remote: Choice,
    pub index_txt_remote: Frame,
    pub start_btn_remote: Button,
    pub stop_btn_remote: Button,
    pub del_btn_remote: Button,
    pub name_iuput_remote: Input,
    pub remote_listen_addr_box: Frame,
    pub real_service_addr_box_remote: Frame,
    pub local_host_input: Input,
    pub img_left_arrow1: Frame,
    pub img_left_arrow2: Frame,
    pub img_left_arrow3: Frame,
    pub forward_port_iuput_remote: IntInput,
    pub local_port_input: Input,
    pub remote_img_firewall: Frame,
    pub menu: MenuBar,
    pub add_local_tunnel_btn: Button,
    pub add_remote_tunnel_btn: Button,
}


impl BasicView {
    pub fn make_window() -> Self {
	let mut main_window = Window::new(363, 175, 960, 400, None);
	main_window.end();
	main_window.set_color(Color::by_index(7));
	main_window.size_range(960, 200, 960, 900);
	main_window.show();
	let mut scroll_view = Scroll::new(0, 50, 960, 320, None);
	scroll_view.end();
	scroll_view.set_frame(FrameType::FlatBox);
	scroll_view.set_color(Color::by_index(7));
	scroll_view.set_label_font(Font::by_index(1));
	scroll_view.set_align(unsafe {std::mem::transmute(0)});
	main_window.add(&scroll_view);
	let mut local_tunnel_group = Group::new(15, 60, 913, 130, "g_id");
	local_tunnel_group.end();
	local_tunnel_group.set_frame(FrameType::UpBox);
	local_tunnel_group.set_color(Color::by_index(54));
	local_tunnel_group.set_label_color(Color::by_index(54));
	local_tunnel_group.set_align(unsafe {std::mem::transmute(0)});
	scroll_view.add(&local_tunnel_group);
	let mut gray_line = Frame::new(15, 99, 913, 3, None);
	gray_line.set_frame(FrameType::FlatBox);
	gray_line.set_color(Color::by_index(38));
	gray_line.set_label_type(LabelType::None);
	local_tunnel_group.add(&gray_line);
	let mut forward_type_choice = Choice::new(585, 66, 75, 24, None);
	forward_type_choice.end();
	forward_type_choice.set_down_frame(FrameType::BorderBox);
	local_tunnel_group.add(&forward_type_choice);
	let mut index_txt = Frame::new(28, 68, 20, 20, "1");
	index_txt.set_color(Color::by_index(46));
	index_txt.set_label_font(Font::by_index(1));
	index_txt.set_label_color(Color::by_index(229));
	local_tunnel_group.add(&index_txt);
	let mut start_btn = Button::new(795, 67, 24, 24, None);
	start_btn.set_tooltip("start tunnel");
	start_btn.set_image(Some(SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png")));
	start_btn.set_frame(FrameType::FlatBox);
	start_btn.set_color(Color::by_index(255));
	start_btn.set_align(unsafe {std::mem::transmute(16)});
	local_tunnel_group.add(&start_btn);
	let mut stop_btn = Button::new(835, 67, 24, 24, None);
	stop_btn.set_tooltip("stop tunnel");
	stop_btn.set_image(Some(SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png")));
	stop_btn.set_frame(FrameType::FlatBox);
	stop_btn.set_color(Color::by_index(255));
	stop_btn.set_align(unsafe {std::mem::transmute(16)});
	stop_btn.deactivate();
	local_tunnel_group.add(&stop_btn);
	let mut del_btn = Button::new(875, 67, 24, 24, None);
	del_btn.set_tooltip("delete this config");
	del_btn.set_image(Some(SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png")));
	del_btn.set_frame(FrameType::FlatBox);
	del_btn.set_color(Color::by_index(255));
	del_btn.set_align(unsafe {std::mem::transmute(16)});
	del_btn.deactivate();
	local_tunnel_group.add(&del_btn);
	let mut name_iuput = Input::new(55, 66, 480, 24, None);
	name_iuput.set_label_type(LabelType::None);
	local_tunnel_group.add(&name_iuput);
	let mut local_listen_addr_box = Frame::new(140, 131, 145, 35, "local listen addr");
	local_listen_addr_box.set_frame(FrameType::UpBox);
	local_listen_addr_box.set_label_type(LabelType::Embossed);
	local_listen_addr_box.set_label_color(Color::by_index(229));
	local_listen_addr_box.set_align(unsafe {std::mem::transmute(1)});
	local_tunnel_group.add(&local_listen_addr_box);
	let mut remote_addr_box = Frame::new(710, 131, 190, 35, "remote service addr");
	remote_addr_box.set_frame(FrameType::UpBox);
	remote_addr_box.set_label_type(LabelType::Embossed);
	remote_addr_box.set_label_color(Color::by_index(229));
	remote_addr_box.set_align(unsafe {std::mem::transmute(1)});
	local_tunnel_group.add(&remote_addr_box);
	let mut remote_host_input = Input::new(718, 137, 110, 24, None);
	remote_host_input.set_label_type(LabelType::None);
	local_tunnel_group.add(&remote_host_input);
	let mut img_pc = Frame::new(50, 126, 40, 40, "💻");
	img_pc.set_label_size(40);
	local_tunnel_group.add(&img_pc);
	let mut img_right_arrow1 = Frame::new(85, 146, 55, 10, "🠖❯");
	img_right_arrow1.set_frame(FrameType::FlatBox);
	img_right_arrow1.set_color(Color::by_index(53));
	img_right_arrow1.set_label_type(LabelType::Engraved);
	img_right_arrow1.set_label_font(Font::by_index(1));
	img_right_arrow1.set_label_size(15);
	img_right_arrow1.set_label_color(Color::by_index(229));
	local_tunnel_group.add(&img_right_arrow1);
	let mut img_right_arrow2 = Frame::new(285, 146, 90, 10, "🠖❯");
	img_right_arrow2.set_frame(FrameType::FlatBox);
	img_right_arrow2.set_color(Color::by_index(53));
	img_right_arrow2.set_label_type(LabelType::Engraved);
	img_right_arrow2.set_label_font(Font::by_index(1));
	img_right_arrow2.set_label_size(15);
	img_right_arrow2.set_label_color(Color::by_index(229));
	local_tunnel_group.add(&img_right_arrow2);
	let mut img_right_arrow3 = Frame::new(665, 146, 45, 10, "🠖❯");
	img_right_arrow3.set_frame(FrameType::FlatBox);
	img_right_arrow3.set_color(Color::by_index(53));
	img_right_arrow3.set_label_type(LabelType::Engraved);
	img_right_arrow3.set_label_font(Font::by_index(1));
	img_right_arrow3.set_label_size(15);
	img_right_arrow3.set_label_color(Color::by_index(229));
	local_tunnel_group.add(&img_right_arrow3);
	let mut forward_port_input = IntInput::new(205, 137, 70, 24, "0.0.0.0:");
	local_tunnel_group.add(&forward_port_input);
	let mut remote_port_input = Input::new(840, 137, 50, 24, ":");
	local_tunnel_group.add(&remote_port_input);
	let mut ssh_server_box = Frame::new(375, 119, 290, 60, "SSH Server");
	ssh_server_box.set_frame(FrameType::UpBox);
	ssh_server_box.set_label_type(LabelType::Embossed);
	ssh_server_box.set_label_color(Color::by_index(229));
	ssh_server_box.set_align(unsafe {std::mem::transmute(1)});
	local_tunnel_group.add(&ssh_server_box);
	let mut img_firewall = Frame::new(350, 118, 8, 70, "firewall");
	img_firewall.set_frame(FrameType::FlatBox);
	img_firewall.set_color(Color::by_index(92));
	img_firewall.set_label_type(LabelType::Embossed);
	img_firewall.set_label_color(Color::by_index(90));
	img_firewall.set_align(unsafe {std::mem::transmute(1)});
	local_tunnel_group.add(&img_firewall);
	let mut ssh_username_input = Input::new(425, 151, 100, 24, "user:");
	local_tunnel_group.add(&ssh_username_input);
	let mut ssh_server_ip_input = Input::new(425, 123, 100, 24, "host:");
	local_tunnel_group.add(&ssh_server_ip_input);
	let mut ssh_port_input = IntInput::new(565, 123, 85, 24, "port:");
	local_tunnel_group.add(&ssh_port_input);
	let mut ssh_pwd_input = SecretInput::new(565, 151, 85, 24, "pwd:");
	local_tunnel_group.add(&ssh_pwd_input);
	let mut remote_tunnel_group = Group::new(15, 200, 913, 130, "g_id");
	remote_tunnel_group.end();
	remote_tunnel_group.set_frame(FrameType::UpBox);
	remote_tunnel_group.set_color(Color::by_index(54));
	remote_tunnel_group.set_label_color(Color::by_index(54));
	remote_tunnel_group.set_align(unsafe {std::mem::transmute(0)});
	scroll_view.add(&remote_tunnel_group);
	let mut fl2rust_widget_0 = Frame::new(15, 239, 913, 3, None);
	fl2rust_widget_0.set_frame(FrameType::FlatBox);
	fl2rust_widget_0.set_color(Color::by_index(38));
	fl2rust_widget_0.set_label_type(LabelType::None);
	remote_tunnel_group.add(&fl2rust_widget_0);
	let mut forward_type_choice_remote = Choice::new(585, 206, 75, 24, None);
	forward_type_choice_remote.end();
	forward_type_choice_remote.set_down_frame(FrameType::BorderBox);
	remote_tunnel_group.add(&forward_type_choice_remote);
	let mut index_txt_remote = Frame::new(28, 208, 20, 20, "1");
	index_txt_remote.set_color(Color::by_index(46));
	index_txt_remote.set_label_font(Font::by_index(1));
	index_txt_remote.set_label_color(Color::by_index(229));
	remote_tunnel_group.add(&index_txt_remote);
	let mut start_btn_remote = Button::new(795, 207, 24, 24, None);
	start_btn_remote.set_image(Some(SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png")));
	start_btn_remote.set_frame(FrameType::FlatBox);
	start_btn_remote.set_color(Color::by_index(255));
	start_btn_remote.set_align(unsafe {std::mem::transmute(16)});
	remote_tunnel_group.add(&start_btn_remote);
	let mut stop_btn_remote = Button::new(835, 207, 24, 24, None);
	stop_btn_remote.set_image(Some(SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png")));
	stop_btn_remote.set_frame(FrameType::FlatBox);
	stop_btn_remote.set_color(Color::by_index(255));
	stop_btn_remote.set_align(unsafe {std::mem::transmute(16)});
	stop_btn_remote.deactivate();
	remote_tunnel_group.add(&stop_btn_remote);
	let mut del_btn_remote = Button::new(875, 207, 24, 24, None);
	del_btn_remote.set_image(Some(SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png")));
	del_btn_remote.set_frame(FrameType::FlatBox);
	del_btn_remote.set_color(Color::by_index(255));
	del_btn_remote.set_align(unsafe {std::mem::transmute(16)});
	del_btn_remote.deactivate();
	remote_tunnel_group.add(&del_btn_remote);
	let mut name_iuput_remote = Input::new(55, 206, 480, 24, None);
	name_iuput_remote.set_label_type(LabelType::None);
	remote_tunnel_group.add(&name_iuput_remote);
	let mut remote_listen_addr_box = Frame::new(755, 271, 145, 35, "remote listen addr");
	remote_listen_addr_box.set_frame(FrameType::UpBox);
	remote_listen_addr_box.set_label_type(LabelType::Embossed);
	remote_listen_addr_box.set_label_color(Color::by_index(229));
	remote_listen_addr_box.set_align(unsafe {std::mem::transmute(1)});
	remote_tunnel_group.add(&remote_listen_addr_box);
	let mut real_service_addr_box_remote = Frame::new(140, 271, 190, 35, "local service addr");
	real_service_addr_box_remote.set_frame(FrameType::UpBox);
	real_service_addr_box_remote.set_label_type(LabelType::Embossed);
	real_service_addr_box_remote.set_label_color(Color::by_index(229));
	real_service_addr_box_remote.set_align(unsafe {std::mem::transmute(1)});
	remote_tunnel_group.add(&real_service_addr_box_remote);
	let mut local_host_input = Input::new(148, 277, 110, 24, None);
	local_host_input.set_label_type(LabelType::None);
	remote_tunnel_group.add(&local_host_input);
	let mut fl2rust_widget_1 = Frame::new(50, 266, 40, 40, "💻");
	fl2rust_widget_1.set_label_size(40);
	remote_tunnel_group.add(&fl2rust_widget_1);
	let mut img_left_arrow1 = Frame::new(85, 286, 55, 10, "❮🠔");
	img_left_arrow1.set_frame(FrameType::FlatBox);
	img_left_arrow1.set_color(Color::by_index(53));
	img_left_arrow1.set_label_type(LabelType::Engraved);
	img_left_arrow1.set_label_font(Font::by_index(1));
	img_left_arrow1.set_label_size(15);
	img_left_arrow1.set_label_color(Color::by_index(229));
	remote_tunnel_group.add(&img_left_arrow1);
	let mut img_left_arrow2 = Frame::new(330, 286, 50, 10, "❮🠔");
	img_left_arrow2.set_frame(FrameType::FlatBox);
	img_left_arrow2.set_color(Color::by_index(53));
	img_left_arrow2.set_label_type(LabelType::Engraved);
	img_left_arrow2.set_label_font(Font::by_index(1));
	img_left_arrow2.set_label_size(15);
	img_left_arrow2.set_label_color(Color::by_index(229));
	remote_tunnel_group.add(&img_left_arrow2);
	let mut img_left_arrow3 = Frame::new(665, 286, 90, 10, "❮🠔");
	img_left_arrow3.set_frame(FrameType::FlatBox);
	img_left_arrow3.set_color(Color::by_index(53));
	img_left_arrow3.set_label_type(LabelType::Engraved);
	img_left_arrow3.set_label_font(Font::by_index(1));
	img_left_arrow3.set_label_size(15);
	img_left_arrow3.set_label_color(Color::by_index(229));
	remote_tunnel_group.add(&img_left_arrow3);
	let mut forward_port_iuput_remote = IntInput::new(825, 277, 70, 24, "127.0.0.1:");
	remote_tunnel_group.add(&forward_port_iuput_remote);
	let mut local_port_input = Input::new(270, 277, 50, 24, ":");
	remote_tunnel_group.add(&local_port_input);
	let mut fl2rust_widget_2 = Frame::new(375, 259, 290, 60, "SSH Server");
	fl2rust_widget_2.set_frame(FrameType::UpBox);
	fl2rust_widget_2.set_label_type(LabelType::Embossed);
	fl2rust_widget_2.set_label_color(Color::by_index(229));
	fl2rust_widget_2.set_align(unsafe {std::mem::transmute(1)});
	remote_tunnel_group.add(&fl2rust_widget_2);
	let mut fl2rust_widget_3 = Input::new(425, 291, 100, 24, "user:");
	remote_tunnel_group.add(&fl2rust_widget_3);
	let mut fl2rust_widget_4 = Input::new(425, 263, 100, 24, "host:");
	remote_tunnel_group.add(&fl2rust_widget_4);
	let mut fl2rust_widget_5 = IntInput::new(565, 263, 85, 24, "port:");
	remote_tunnel_group.add(&fl2rust_widget_5);
	let mut fl2rust_widget_6 = SecretInput::new(565, 291, 85, 24, "pwd:");
	remote_tunnel_group.add(&fl2rust_widget_6);
	let mut remote_img_firewall = Frame::new(360, 256, 8, 70, "firewall");
	remote_img_firewall.set_frame(FrameType::FlatBox);
	remote_img_firewall.set_color(Color::by_index(92));
	remote_img_firewall.set_label_type(LabelType::Embossed);
	remote_img_firewall.set_label_color(Color::by_index(90));
	remote_img_firewall.set_align(unsafe {std::mem::transmute(1)});
	remote_tunnel_group.add(&remote_img_firewall);
	let mut menu = MenuBar::new(0, 0, 960, 20, None);
	menu.end();
	menu.set_color(Color::by_index(46));
	main_window.add(&menu);
	let mut fl2rust_widget_7 = Button::new(890, 1, 62, 18, "about");
	main_window.add(&fl2rust_widget_7);
	let mut add_local_tunnel_btn = Button::new(2, 1, 100, 18, "Add Local");
	add_local_tunnel_btn.set_frame(FrameType::FlatBox);
	add_local_tunnel_btn.set_color(Color::by_index(228));
	add_local_tunnel_btn.set_label_font(Font::by_index(1));
	add_local_tunnel_btn.set_label_color(Color::by_index(55));
	main_window.add(&add_local_tunnel_btn);
	let mut add_remote_tunnel_btn = Button::new(110, 1, 100, 18, "Add Remote");
	add_remote_tunnel_btn.set_frame(FrameType::FlatBox);
	add_remote_tunnel_btn.set_color(Color::by_index(228));
	add_remote_tunnel_btn.set_label_font(Font::by_index(1));
	add_remote_tunnel_btn.set_label_color(Color::by_index(55));
	main_window.add(&add_remote_tunnel_btn);
	let mut fl2rust_widget_8 = Group::new(0, 20, 960, 30, None);
	fl2rust_widget_8.end();
	fl2rust_widget_8.set_frame(FrameType::UpBox);
	fl2rust_widget_8.set_color(Color::by_index(46));
	main_window.add(&fl2rust_widget_8);
	let mut fl2rust_widget_9 = Frame::new(75, 25, 45, 20, "Name");
	fl2rust_widget_8.add(&fl2rust_widget_9);
	let mut fl2rust_widget_10 = Frame::new(600, 25, 40, 20, "Type");
	fl2rust_widget_8.add(&fl2rust_widget_10);
	let mut fl2rust_widget_11 = Frame::new(785, 25, 40, 20, "Start");
	fl2rust_widget_8.add(&fl2rust_widget_11);
	let mut fl2rust_widget_12 = Frame::new(825, 25, 40, 20, "Stop");
	fl2rust_widget_8.add(&fl2rust_widget_12);
	let mut fl2rust_widget_13 = Frame::new(25, 25, 30, 20, "No.");
	fl2rust_widget_8.add(&fl2rust_widget_13);
	let mut fl2rust_widget_14 = Frame::new(872, 25, 28, 20, "Del");
	fl2rust_widget_8.add(&fl2rust_widget_14);
	Self { main_window, scroll_view, local_tunnel_group, gray_line, forward_type_choice, index_txt, start_btn, stop_btn, del_btn, name_iuput, local_listen_addr_box, remote_addr_box, remote_host_input, img_pc, img_right_arrow1, img_right_arrow2, img_right_arrow3, forward_port_input, remote_port_input, ssh_server_box, img_firewall, ssh_username_input, ssh_server_ip_input, ssh_port_input, ssh_pwd_input, remote_tunnel_group, forward_type_choice_remote, index_txt_remote, start_btn_remote, stop_btn_remote, del_btn_remote, name_iuput_remote, remote_listen_addr_box, real_service_addr_box_remote, local_host_input, img_left_arrow1, img_left_arrow2, img_left_arrow3, forward_port_iuput_remote, local_port_input, remote_img_firewall, menu, add_local_tunnel_btn, add_remote_tunnel_btn, }
    }
}


