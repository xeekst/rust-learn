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

use crate::basic_view::BasicView;
use crate::MsgType;
use crate::UiMessage;
use crate::RWLOCK_MSG_CHANNEL;

pub struct SSHTunnelView {
    pub basic_view: BasicView,
    pub tunnel_rows: Vec<(Group, Input, Input, Input, Input, Input, Button, Button)>,
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
        let mut add_btn = self.basic_view.menu.find_item("+").unwrap();
        add_btn.set_callback(|_| {
            Self::send(UiMessage {
                msg_type: MsgType::AddTunnelRow,
                msg: String::from(""),
            })
        });
    }

    pub fn add_ssh_tunnel_row(&mut self) {
        let (x, y) = if self.tunnel_rows.len() > 0 {
            (
                self.tunnel_rows.last().unwrap().0.x(),
                self.tunnel_rows.last().unwrap().0.y(),
            )
        } else {
            (15, 10)
        };

        let mut tunnel = Group::new(x, y + 40, 763, 35, None);
        tunnel.end();
        tunnel.set_color(Color::by_index(53));
        tunnel.set_align(unsafe { std::mem::transmute(0) });
        tunnel.set_frame(FrameType::BorderBox);
        self.basic_view.scroll_view.add(&tunnel);

        let mut start_btn = Button::new(197, y + 42, 24, 24, None);
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
        let mut stop_btn = Button::new(232, y + 42, 24, 24, None);
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
        let mut name_input = Input::new(22, y + 44, 95, 24, None);
        name_input.set_label_type(LabelType::None);
        tunnel.add(&name_input);
        let mut type_input = Input::new(127, y + 44, 50, 24, None);
        type_input.set_label_type(LabelType::None);
        tunnel.add(&type_input);
        let mut forward_port = Input::new(292, y + 44, 50, 24, None);
        forward_port.set_label_type(LabelType::None);
        tunnel.add(&forward_port);
        let mut dst_host_port = Input::new(374, y + 44, 110, 24, None);
        dst_host_port.set_label_type(LabelType::None);
        tunnel.add(&dst_host_port);
        let mut ssh_server_input = Input::new(507, y + 44, 228, 24, None);
        ssh_server_input.set_label_type(LabelType::None);
        tunnel.add(&ssh_server_input);

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
            name_input,
            type_input,
            forward_port,
            dst_host_port,
            ssh_server_input,
            start_btn,
            stop_btn,
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
