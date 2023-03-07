use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image;
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

use crate::basic_view::BasicView;
use crate::MsgType;
use crate::UiMessage;
use crate::RWLOCK_MSG_CHANNEL;

pub struct SSHTunnelView {
    pub basic_view: BasicView,

    pub tunnel_rows: Vec<(
        Group,
        CheckButton,
        Frame,
        Input,
        MenuButton,
        Button,
        Button,
        ValueInput,
        Input,
        Input,
        Input,
        ValueInput,
        SecretInput,
    )>,
}

impl SSHTunnelView {
    pub fn new() -> Self {
        let view = BasicView::make_window();
        SSHTunnelView {
            basic_view: view,
            tunnel_rows: vec![],
        }
    }

    pub fn init(&mut self) {
        self.basic_view
            .main_window
            .resize_callback(|_, x, y, w, h| {
                Self::send(UiMessage {
                    msg_type: MsgType::ResizeMainWindow,
                    msg: format!("{x}|{y}|{w}|{h}"),
                })
            });
        let image = image::PngImage::load("asset/network.png").unwrap();
        self.basic_view.main_window.set_icon(Some(image));
        let mut add_btn = self.basic_view.menu.find_item("+").unwrap();
        add_btn.set_callback(|_| {
            Self::send(UiMessage {
                msg_type: MsgType::AddTunnelRow,
                msg: String::from(""),
            })
        });
        
    }

    pub fn add_ssh_tunnel_row(&mut self) {
        let y = if self.tunnel_rows.len() > 0 {
            self.tunnel_rows.last().unwrap().2.y() +  45
        } else {
            self.basic_view.name_iuput.y()
        };

        let mut tunnel = Group::new(
            self.basic_view.tunnel_row.x(),
            if self.tunnel_rows.len() > 0 {
                self.tunnel_rows.last().unwrap().0.y() + 45
            } else {
                self.basic_view.tunnel_row.y()
            },
            self.basic_view.tunnel_row.w(),
            self.basic_view.tunnel_row.h(),
            None,
        );
        tunnel.end();
        // let border = color = if self.tunnel_rows.len() % 2 == 0 {
        //     self.basic_view.tunnel_row.color()
        // } else {
        //     Color::Blue
        // };
        tunnel.set_color(self.basic_view.tunnel_row.color());
        tunnel.set_align(unsafe { std::mem::transmute(0) });
        tunnel.set_frame(FrameType::BorderBox);
        self.basic_view.scroll_view.add(&tunnel);

        let mut index_txt = Frame::new(
            self.basic_view.index_txt.x(),
            y,
            self.basic_view.index_txt.w(),
            self.basic_view.index_txt.h(),
            "0",
        );
        index_txt.set_label(&format!("{}", self.tunnel_rows.len()));
        index_txt.set_color(Color::by_index(46));
        index_txt.set_label_font(Font::by_index(1));
        index_txt.set_label_color(Color::by_index(229));
        tunnel.add(&index_txt);

        let mut forward_type_choice = MenuButton::new(
            self.basic_view.forward_type_choice.x(),
            y,
            self.basic_view.forward_type_choice.w(),
            self.basic_view.forward_type_choice.h(),
            "menu",
        );
        forward_type_choice.end();
        tunnel.add(&forward_type_choice);

        let mut start_btn = Button::new(
            self.basic_view.start_btn.x(),
            y - 2,
            self.basic_view.start_btn.w(),
            self.basic_view.start_btn.h(),
            None,
        );
        start_btn.set_image(Some(
            SharedImage::load("asset\\play.png")
                .expect("Could not find image: ..\\asset\\play.png"),
        ));
        start_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_play.png")
                .expect("Could not find image: ..\\asset\\inactive_play.png"),
        ));
        start_btn.set_frame(FrameType::FlatBox);
        start_btn.set_color(Color::by_index(255));
        start_btn.set_align(unsafe { std::mem::transmute(16) });
        start_btn.set_tooltip("start this ssh tunnel.");
        tunnel.add(&start_btn);
        let mut stop_btn = Button::new(
            self.basic_view.stop_btn.x(),
            y - 2,
            self.basic_view.stop_btn.width(),
            self.basic_view.stop_btn.h(),
            None,
        );
        stop_btn.set_image(Some(
            SharedImage::load("asset\\stop.png")
                .expect("Could not find image: ..\\asset\\stop.png"),
        ));
        stop_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_stop.png")
                .expect("Could not find image: ..\\asset\\inactive_stop.png"),
        ));
        stop_btn.set_frame(FrameType::FlatBox);
        stop_btn.set_color(Color::by_index(255));
        stop_btn.set_tooltip("stop this ssh tunnel.");
        stop_btn.set_align(unsafe { std::mem::transmute(16) });
        stop_btn.deactivate();
        tunnel.add(&stop_btn);

        let mut name_iuput = Input::new(
            self.basic_view.name_iuput.x(),
            y,
            self.basic_view.name_iuput.w(),
            self.basic_view.name_iuput.h(),
            None,
        );
        name_iuput.set_label_type(LabelType::None);
        tunnel.add(&name_iuput);
        let mut forward_port_iuput = ValueInput::new(
            self.basic_view.forward_port_iuput.x(),
            y,
            self.basic_view.forward_port_iuput.w(),
            self.basic_view.forward_port_iuput.h(),
            None,
        );
        forward_port_iuput.set_label_type(LabelType::None);
        tunnel.add(&forward_port_iuput);
        let mut dst_server_port_input = Input::new(
            self.basic_view.dst_server_port_input.x(),
            y,
            self.basic_view.dst_server_port_input.w(),
            self.basic_view.dst_server_port_input.h(),
            None,
        );
        dst_server_port_input.set_label_type(LabelType::None);
        tunnel.add(&dst_server_port_input);
        let mut ssh_username_iuput = Input::new(
            self.basic_view.ssh_username_iuput.x(),
            y,
            self.basic_view.ssh_username_iuput.w(),
            self.basic_view.ssh_username_iuput.h(),
            None,
        );
        ssh_username_iuput.set_label_type(LabelType::None);
        tunnel.add(&ssh_username_iuput);
        let mut ssh_server_ip_iuput = Input::new(
            self.basic_view.ssh_server_ip_iuput.x(),
            y,
            self.basic_view.ssh_server_ip_iuput.w(),
            self.basic_view.ssh_server_ip_iuput.h(),
            None,
        );
        ssh_server_ip_iuput.set_label_type(LabelType::None);
        tunnel.add(&ssh_server_ip_iuput);
        let mut ssh_port_iuput = ValueInput::new(
            self.basic_view.ssh_port_iuput.x(),
            y,
            self.basic_view.ssh_port_iuput.w(),
            self.basic_view.ssh_port_iuput.h(),
            None,
        );
        ssh_port_iuput.set_label_type(LabelType::None);
        tunnel.add(&ssh_port_iuput);
        let mut pwd_input = SecretInput::new(
            self.basic_view.pwd_input.x(),
            y,
            self.basic_view.pwd_input.w(),
            self.basic_view.pwd_input.h(),
            None,
        );
        pwd_input.set_label_type(LabelType::None);
        tunnel.add(&pwd_input);
        let mut fl2rust_widget_10 = Frame::new(600, y, 20, 20, "@@");
        tunnel.add(&fl2rust_widget_10);
        let mut fl2rust_widget_11 = Frame::new(705, y, 10, 20, ":");
        fl2rust_widget_11.set_label_font(Font::by_index(1));
        tunnel.add(&fl2rust_widget_11);
        let mut check_box = CheckButton::new(
            self.basic_view.check_box.x(),
            y + 4,
            self.basic_view.check_box.w(),
            self.basic_view.check_box.h(),
            None,
        );
        check_box.set_down_frame(FrameType::DownBox);
        check_box.set_color(Color::by_index(229));
        check_box.set_selection_color(Color::by_index(228));
        check_box.set_label_type(LabelType::None);
        check_box.set_label_color(Color::by_index(229));
        tunnel.add(&check_box);

        let start_btn_index = (self.tunnel_rows.len()).to_string();

        start_btn.set_callback(move |b| {
            Self::send(UiMessage {
                msg_type: MsgType::StartTunnel,
                msg: start_btn_index.to_owned(),
            })
        });

        let stop_btn_index = (self.tunnel_rows.len()).to_string();
        stop_btn.set_callback(move |_| {
            Self::send(UiMessage {
                msg_type: MsgType::StopTunnel,
                msg: stop_btn_index.to_owned(),
            })
        });

        self.tunnel_rows.push((
            tunnel,
            check_box,
            index_txt,
            name_iuput,
            forward_type_choice,
            start_btn,
            stop_btn,
            forward_port_iuput,
            dst_server_port_input,
            ssh_username_iuput,
            ssh_server_ip_iuput,
            ssh_port_iuput,
            pwd_input,
        ));
    }

    fn send(m: UiMessage) {
        match &RWLOCK_MSG_CHANNEL.read() {
            Ok(channel) => {
                let sender = &channel.0;
                sender.send(m);
            }
            Err(err) => panic!("get message channel sender error:{}", err),
        }
    }
}

fn start_ssh_tunnel(tunnel_id: &str) {}

fn stop_ssh_tunnel(tunnel_id: &str) {}
