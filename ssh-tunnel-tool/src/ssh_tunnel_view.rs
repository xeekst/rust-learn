use std::collections::HashMap;

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
use crate::ssh_tunnel::SSHTunnel;
use crate::ssh_tunnel::TunnelType;
use crate::MsgType;
use crate::UiMessage;
use crate::RWLOCK_MSG_CHANNEL;
use anyhow::{anyhow, Result};

pub struct SSHTunnelView {
    pub basic_view: BasicView,

    pub tunnel_rows: Vec<SSHTunnelRow>,
}

pub struct SSHTunnelRow {
    pub id: i32,
    pub tunnel_type: TunnelType,

    pub row_group: Group,
    pub index_txt: Frame,
    pub name_iuput: Input,
    pub forward_type_choice: Choice,
    pub start_btn: Button,
    pub stop_btn: Button,
    pub del_btn: Button,

    pub img_gray_line: Frame,
    pub img_pc: Frame,
    pub img_arrow1: Frame,
    pub img_arrow2: Frame,
    pub img_arrow3: Frame,
    pub img_firewall: Frame,
    pub listen_addr_box: Frame,
    pub ssh_server_box: Frame,
    pub real_service_addr_box: Frame,

    // 指新创建的一个监听TCP服务地址
    // 类型为: Local =>  是在本机监听的一个TCP端口，供本机服务访问该端口
    // 类型为: Remote => 是在远程ssh服务器监听的一个TCP端口，以供ssh服务器可以访问该端口
    pub listen_port_input: Input,
    pub ssh_server_host_input: Input,
    pub ssh_server_port_input: Input,
    pub ssh_server_username_input: Input,
    pub ssh_server_pwd_input: Input,
    // real service 是指实际提供服务 ip:port
    // 类型为: Local =>  是在远程服务器上的某个服务
    // 类型为: Remote => 是在本机上可访问的某个服务
    pub real_service_host_input: Input,
    pub real_service_port_input: Input,
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
                send(UiMessage {
                    msg_type: MsgType::ResizeMainWindow,
                    msg: format!("{x}|{y}|{w}|{h}"),
                })
            });
        let image = image::PngImage::load("asset/network.png").unwrap();
        self.basic_view.main_window.set_icon(Some(image));
        self.basic_view.add_local_tunnel_btn.set_callback(|b| {
            send(UiMessage {
                msg_type: MsgType::AddLocalTunnelRow,
                msg: String::from(""),
            })
        });
        self.basic_view.add_remote_tunnel_btn.set_callback(|b| {
            send(UiMessage {
                msg_type: MsgType::AddRemoteTunnelRow,
                msg: String::from(""),
            })
        });
        self.basic_view.local_tunnel_group.hide();
        self.basic_view.remote_tunnel_group.hide();
    }

    pub fn add_ssh_remote_tunnel_row(&mut self) {
        let idx = self.tunnel_rows.len() as i32;
        let id = idx;
        let y_offset = (self.basic_view.local_tunnel_group.h() + 10) * idx;
        let mut tunnel_group = Group::new(
            self.basic_view.local_tunnel_group.x(),
            self.basic_view.local_tunnel_group.y() + y_offset,
            self.basic_view.local_tunnel_group.w(),
            self.basic_view.local_tunnel_group.h(),
            "",
        );
        tunnel_group.set_frame(FrameType::UpBox);
        tunnel_group.set_color(Color::by_index(54));
        tunnel_group.set_label_color(Color::by_index(54));
        tunnel_group.set_align(unsafe { std::mem::transmute(0) });
        self.basic_view.scroll_view.add(&tunnel_group);

        let mut index_txt = Frame::new(
            self.basic_view.index_txt.x(),
            self.basic_view.index_txt.y() + y_offset,
            self.basic_view.index_txt.w(),
            self.basic_view.index_txt.h(),
            None,
        );
        index_txt.set_color(Color::by_index(46));
        index_txt.set_label_font(Font::by_index(1));
        index_txt.set_label_color(Color::by_index(229));
        index_txt.set_label(&format!("{idx}"));
        tunnel_group.add(&index_txt);

        let name_iuput = Input::new(
            self.basic_view.name_iuput.x(),
            self.basic_view.name_iuput.y() + y_offset,
            self.basic_view.name_iuput.w(),
            self.basic_view.name_iuput.h(),
            "",
        );
        tunnel_group.add(&name_iuput);

        let mut forward_type_choice = Choice::new(
            self.basic_view.forward_type_choice.x(),
            self.basic_view.forward_type_choice.y() + y_offset,
            self.basic_view.forward_type_choice.w(),
            self.basic_view.forward_type_choice.h(),
            "",
        );
        forward_type_choice.add_choice("Local");
        forward_type_choice.add_choice("Remote");
        tunnel_group.add(&forward_type_choice);

        let mut start_btn = Button::new(
            self.basic_view.start_btn.x(),
            self.basic_view.start_btn.y() + y_offset,
            self.basic_view.start_btn.w(),
            self.basic_view.start_btn.h(),
            None,
        );
        start_btn.set_tooltip("start tunnel");
        start_btn.set_image(Some(
            SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png"),
        ));
        start_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_play.png")
                .expect("Could not find image: asset\\inactive_play.png"),
        ));
        start_btn.set_frame(FrameType::FlatBox);
        start_btn.set_color(Color::by_index(255));
        start_btn.set_align(unsafe { std::mem::transmute(16) });
        start_btn.set_callback(move |b| {
            send(UiMessage {
                msg_type: MsgType::StartTunnel,
                msg: id.to_string(),
            })
        });
        tunnel_group.add(&start_btn);

        let mut stop_btn = Button::new(
            self.basic_view.stop_btn.x(),
            self.basic_view.stop_btn.y() + y_offset,
            self.basic_view.stop_btn.w(),
            self.basic_view.stop_btn.h(),
            None,
        );
        stop_btn.set_tooltip("stop tunnel");
        stop_btn.set_image(Some(
            SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png"),
        ));
        stop_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_stop.png")
                .expect("Could not find image: asset\\inactive_stop.png"),
        ));
        stop_btn.set_frame(FrameType::FlatBox);
        stop_btn.set_color(Color::by_index(255));
        stop_btn.set_align(unsafe { std::mem::transmute(16) });
        stop_btn.set_callback(move |b| {
            send(UiMessage {
                msg_type: MsgType::StopTunnel,
                msg: id.to_string(),
            })
        });
        stop_btn.deactivate();
        tunnel_group.add(&stop_btn);

        let mut del_btn = Button::new(
            self.basic_view.del_btn.x(),
            self.basic_view.del_btn.y() + y_offset,
            self.basic_view.del_btn.w(),
            self.basic_view.del_btn.h(),
            None,
        );
        del_btn.set_tooltip("delete this tunnel");
        del_btn.set_image(Some(
            SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png"),
        ));
        del_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_del.png")
                .expect("Could not find image: asset\\inactive_del.png"),
        ));
        del_btn.set_frame(FrameType::FlatBox);
        del_btn.set_color(Color::by_index(255));
        del_btn.set_align(unsafe { std::mem::transmute(16) });
        del_btn.set_callback(move |b| {
            send(UiMessage {
                msg_type: MsgType::DeleteTunnel,
                msg: id.to_string(),
            })
        });
        tunnel_group.add(&del_btn);

        let mut img_gray_line = Frame::new(
            self.basic_view.gray_line.x(),
            self.basic_view.gray_line.y() + y_offset,
            self.basic_view.gray_line.w(),
            self.basic_view.gray_line.h(),
            None,
        );
        img_gray_line.set_frame(FrameType::FlatBox);
        img_gray_line.set_color(Color::by_index(38));
        img_gray_line.set_label_type(LabelType::None);
        tunnel_group.add(&img_gray_line);

        let mut img_pc = Frame::new(
            self.basic_view.img_pc.x(),
            self.basic_view.img_pc.y() + y_offset,
            self.basic_view.img_pc.w(),
            self.basic_view.img_pc.h(),
            "💻",
        );
        img_pc.set_label_size(40);

        tunnel_group.add(&img_pc);

        let mut img_arrow1 = Frame::new(
            self.basic_view.img_left_arrow1.x(),
            self.basic_view.img_right_arrow1.y() + y_offset,
            self.basic_view.img_left_arrow1.w(),
            self.basic_view.img_left_arrow1.h(),
            "❮🠔",
        );
        img_arrow1.set_frame(FrameType::FlatBox);
        img_arrow1.set_color(Color::by_index(53));
        img_arrow1.set_label_type(LabelType::Engraved);
        img_arrow1.set_label_font(Font::by_index(1));
        img_arrow1.set_label_size(15);
        img_arrow1.set_label_color(Color::by_index(229));
        tunnel_group.add(&img_arrow1);
        let mut img_arrow2 = Frame::new(
            self.basic_view.img_left_arrow2.x(),
            self.basic_view.img_right_arrow2.y() + y_offset,
            self.basic_view.img_left_arrow2.w(),
            self.basic_view.img_left_arrow2.h(),
            "❮🠔",
        );
        img_arrow2.set_frame(FrameType::FlatBox);
        img_arrow2.set_color(Color::by_index(53));
        img_arrow2.set_label_type(LabelType::Engraved);
        img_arrow2.set_label_font(Font::by_index(1));
        img_arrow2.set_label_size(15);
        img_arrow2.set_label_color(Color::by_index(229));
        tunnel_group.add(&img_arrow2);
        let mut img_arrow3 = Frame::new(
            self.basic_view.img_left_arrow3.x(),
            self.basic_view.img_right_arrow3.y() + y_offset,
            self.basic_view.img_left_arrow3.w(),
            self.basic_view.img_left_arrow3.h(),
            "❮🠔",
        );
        img_arrow3.set_frame(FrameType::FlatBox);
        img_arrow3.set_color(Color::by_index(53));
        img_arrow3.set_label_type(LabelType::Engraved);
        img_arrow3.set_label_font(Font::by_index(1));
        img_arrow3.set_label_size(15);
        img_arrow3.set_label_color(Color::by_index(229));
        tunnel_group.add(&img_arrow3);

        let mut img_firewall = Frame::new(
            self.basic_view.remote_img_firewall.x(),
            self.basic_view.img_firewall.y() + y_offset,
            self.basic_view.remote_img_firewall.w(),
            self.basic_view.remote_img_firewall.h(),
            "firewall",
        );
        img_firewall.set_frame(FrameType::FlatBox);
        img_firewall.set_color(Color::by_index(92));
        img_firewall.set_label_type(LabelType::Embossed);
        img_firewall.set_label_color(Color::by_index(90));
        img_firewall.set_align(unsafe { std::mem::transmute(1) });
        tunnel_group.add(&img_firewall);

        let mut real_service_addr_box = Frame::new(
            self.basic_view.real_service_addr_box_remote.x(),
            self.basic_view.remote_addr_box.y() + y_offset,
            self.basic_view.real_service_addr_box_remote.w(),
            self.basic_view.real_service_addr_box_remote.h(),
            "local service addr",
        );
        real_service_addr_box.set_frame(FrameType::UpBox);
        real_service_addr_box.set_label_type(LabelType::Embossed);
        real_service_addr_box.set_label_color(Color::by_index(229));
        real_service_addr_box.set_align(unsafe { std::mem::transmute(1) });
        tunnel_group.add(&real_service_addr_box);

        let real_service_host_input = Input::new(
            self.basic_view.local_host_input.x(),
            self.basic_view.remote_host_input.y() + y_offset,
            self.basic_view.local_host_input.w(),
            self.basic_view.local_host_input.h(),
            "",
        );
        tunnel_group.add(&real_service_host_input);
        let real_service_port_input = Input::new(
            self.basic_view.local_port_input.x(),
            self.basic_view.remote_port_input.y() + y_offset,
            self.basic_view.local_port_input.w(),
            self.basic_view.local_port_input.h(),
            ":",
        );
        tunnel_group.add(&real_service_port_input);

        let mut ssh_server_box = Frame::new(
            self.basic_view.ssh_server_box.x(),
            self.basic_view.ssh_server_box.y() + y_offset,
            self.basic_view.ssh_server_box.w(),
            self.basic_view.ssh_server_box.h(),
            "SSH Server",
        );
        ssh_server_box.set_frame(FrameType::UpBox);
        ssh_server_box.set_label_type(LabelType::Embossed);
        ssh_server_box.set_label_color(Color::by_index(229));
        ssh_server_box.set_align(unsafe { std::mem::transmute(1) });
        tunnel_group.add(&ssh_server_box);

        let ssh_server_host_input = Input::new(
            self.basic_view.ssh_server_ip_input.x(),
            self.basic_view.ssh_server_ip_input.y() + y_offset,
            self.basic_view.ssh_server_ip_input.w(),
            self.basic_view.ssh_server_ip_input.h(),
            "host:",
        );
        tunnel_group.add(&ssh_server_host_input);

        let ssh_server_port_input = Input::new(
            self.basic_view.ssh_port_input.x(),
            self.basic_view.ssh_port_input.y() + y_offset,
            self.basic_view.ssh_port_input.w(),
            self.basic_view.ssh_port_input.h(),
            "port:",
        );
        tunnel_group.add(&ssh_server_port_input);

        let ssh_server_username_input = Input::new(
            self.basic_view.ssh_username_input.x(),
            self.basic_view.ssh_username_input.y() + y_offset,
            self.basic_view.ssh_username_input.w(),
            self.basic_view.ssh_username_input.h(),
            "user:",
        );
        tunnel_group.add(&ssh_server_username_input);

        let ssh_server_pwd_input = Input::new(
            self.basic_view.ssh_pwd_input.x(),
            self.basic_view.ssh_pwd_input.y() + y_offset,
            self.basic_view.ssh_pwd_input.w(),
            self.basic_view.ssh_pwd_input.h(),
            "pwd:",
        );
        tunnel_group.add(&ssh_server_pwd_input);

        let mut listen_addr_box = Frame::new(
            self.basic_view.remote_listen_addr_box.x(),
            self.basic_view.local_listen_addr_box.y() + y_offset,
            self.basic_view.remote_listen_addr_box.w(),
            self.basic_view.remote_listen_addr_box.h(),
            "remote listen addr",
        );
        listen_addr_box.set_frame(FrameType::UpBox);
        listen_addr_box.set_label_type(LabelType::Embossed);
        listen_addr_box.set_label_color(Color::by_index(229));
        listen_addr_box.set_align(unsafe { std::mem::transmute(1) });
        tunnel_group.add(&listen_addr_box);

        let listen_port_input = Input::new(
            self.basic_view.forward_port_iuput_remote.x(),
            self.basic_view.forward_port_input.y() + y_offset,
            self.basic_view.forward_port_iuput_remote.w(),
            self.basic_view.forward_port_iuput_remote.h(),
            "127.0.0.1:",
        );
        tunnel_group.add(&listen_port_input);

        let row = SSHTunnelRow {
            id: idx,
            tunnel_type: TunnelType::Remote,
            row_group: tunnel_group,
            index_txt,
            name_iuput,
            forward_type_choice,
            start_btn,
            stop_btn,
            del_btn,
            img_gray_line,
            img_pc,
            img_arrow1,
            img_arrow2,
            img_arrow3,
            img_firewall,
            listen_addr_box,
            ssh_server_box,
            real_service_addr_box,
            listen_port_input,
            ssh_server_host_input,
            ssh_server_port_input,
            ssh_server_username_input,
            ssh_server_pwd_input,
            real_service_host_input,
            real_service_port_input,
        };

        self.tunnel_rows.push(row);
    }

    pub fn add_ssh_local_tunnel_row(&mut self) {
        let idx = self.tunnel_rows.len() as i32;
        let id = idx;
        let y_offset = (self.basic_view.local_tunnel_group.h() + 10) * idx;
        let mut local_tunnel_group = Group::new(
            self.basic_view.local_tunnel_group.x(),
            self.basic_view.local_tunnel_group.y() + y_offset,
            self.basic_view.local_tunnel_group.w(),
            self.basic_view.local_tunnel_group.h(),
            "",
        );
        local_tunnel_group.set_frame(FrameType::UpBox);
        local_tunnel_group.set_color(Color::by_index(54));
        local_tunnel_group.set_label_color(Color::by_index(54));
        local_tunnel_group.set_align(unsafe { std::mem::transmute(0) });
        self.basic_view.scroll_view.add(&local_tunnel_group);

        let mut index_txt = Frame::new(
            self.basic_view.index_txt.x(),
            self.basic_view.index_txt.y() + y_offset,
            self.basic_view.index_txt.w(),
            self.basic_view.index_txt.h(),
            None,
        );
        index_txt.set_color(Color::by_index(46));
        index_txt.set_label_font(Font::by_index(1));
        index_txt.set_label_color(Color::by_index(229));
        index_txt.set_label(&format!("{idx}"));
        local_tunnel_group.add(&index_txt);

        let name_iuput = Input::new(
            self.basic_view.name_iuput.x(),
            self.basic_view.name_iuput.y() + y_offset,
            self.basic_view.name_iuput.w(),
            self.basic_view.name_iuput.h(),
            "",
        );
        local_tunnel_group.add(&name_iuput);

        let mut forward_type_choice = Choice::new(
            self.basic_view.forward_type_choice.x(),
            self.basic_view.forward_type_choice.y() + y_offset,
            self.basic_view.forward_type_choice.w(),
            self.basic_view.forward_type_choice.h(),
            "",
        );
        forward_type_choice.add_choice("Local");
        forward_type_choice.add_choice("Remote");
        local_tunnel_group.add(&forward_type_choice);

        let mut start_btn = Button::new(
            self.basic_view.start_btn.x(),
            self.basic_view.start_btn.y() + y_offset,
            self.basic_view.start_btn.w(),
            self.basic_view.start_btn.h(),
            None,
        );
        start_btn.set_tooltip("start tunnel");
        start_btn.set_image(Some(
            SharedImage::load("asset\\play.png").expect("Could not find image: asset\\play.png"),
        ));
        start_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_play.png")
                .expect("Could not find image: asset\\inactive_play.png"),
        ));
        start_btn.set_frame(FrameType::FlatBox);
        start_btn.set_color(Color::by_index(255));
        start_btn.set_align(unsafe { std::mem::transmute(16) });
        local_tunnel_group.add(&start_btn);

        let mut stop_btn = Button::new(
            self.basic_view.stop_btn.x(),
            self.basic_view.stop_btn.y() + y_offset,
            self.basic_view.stop_btn.w(),
            self.basic_view.stop_btn.h(),
            None,
        );
        stop_btn.set_tooltip("stop tunnel");
        stop_btn.set_image(Some(
            SharedImage::load("asset\\stop.png").expect("Could not find image: asset\\stop.png"),
        ));
        stop_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_stop.png")
                .expect("Could not find image: asset\\inactive_stop.png"),
        ));
        stop_btn.set_frame(FrameType::FlatBox);
        stop_btn.set_color(Color::by_index(255));
        stop_btn.set_align(unsafe { std::mem::transmute(16) });
        stop_btn.deactivate();
        local_tunnel_group.add(&stop_btn);

        let mut del_btn = Button::new(
            self.basic_view.del_btn.x(),
            self.basic_view.del_btn.y() + y_offset,
            self.basic_view.del_btn.w(),
            self.basic_view.del_btn.h(),
            None,
        );
        println!(
            "self.basic_view.del_btn.w():{}",
            self.basic_view.del_btn.w()
        );
        del_btn.set_tooltip("delete this tunnel");
        del_btn.set_image(Some(
            SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png"),
        ));
        del_btn.set_deimage(Some(
            SharedImage::load("asset\\inactive_del.png")
                .expect("Could not find image: asset\\inactive_del.png"),
        ));
        del_btn.set_frame(FrameType::FlatBox);
        del_btn.set_color(Color::by_index(255));
        del_btn.set_align(unsafe { std::mem::transmute(16) });
        local_tunnel_group.add(&del_btn);

        let mut img_gray_line = Frame::new(
            self.basic_view.gray_line.x(),
            self.basic_view.gray_line.y() + y_offset,
            self.basic_view.gray_line.w(),
            self.basic_view.gray_line.h(),
            None,
        );
        img_gray_line.set_frame(FrameType::FlatBox);
        img_gray_line.set_color(Color::by_index(38));
        img_gray_line.set_label_type(LabelType::None);
        local_tunnel_group.add(&img_gray_line);

        let mut img_pc = Frame::new(
            self.basic_view.img_pc.x(),
            self.basic_view.img_pc.y() + y_offset,
            self.basic_view.img_pc.w(),
            self.basic_view.img_pc.h(),
            "💻",
        );
        img_pc.set_label_size(40);

        local_tunnel_group.add(&img_pc);

        let mut img_arrow1 = Frame::new(
            self.basic_view.img_right_arrow1.x(),
            self.basic_view.img_right_arrow1.y() + y_offset,
            self.basic_view.img_right_arrow1.w(),
            self.basic_view.img_right_arrow1.h(),
            "🠖❯",
        );
        img_arrow1.set_frame(FrameType::FlatBox);
        img_arrow1.set_color(Color::by_index(53));
        img_arrow1.set_label_type(LabelType::Engraved);
        img_arrow1.set_label_font(Font::by_index(1));
        img_arrow1.set_label_size(15);
        img_arrow1.set_label_color(Color::by_index(229));
        local_tunnel_group.add(&img_arrow1);
        let mut img_arrow2 = Frame::new(
            self.basic_view.img_right_arrow2.x(),
            self.basic_view.img_right_arrow2.y() + y_offset,
            self.basic_view.img_right_arrow2.w(),
            self.basic_view.img_right_arrow2.h(),
            "🠖❯",
        );
        img_arrow2.set_frame(FrameType::FlatBox);
        img_arrow2.set_color(Color::by_index(53));
        img_arrow2.set_label_type(LabelType::Engraved);
        img_arrow2.set_label_font(Font::by_index(1));
        img_arrow2.set_label_size(15);
        img_arrow2.set_label_color(Color::by_index(229));
        local_tunnel_group.add(&img_arrow2);
        let mut img_arrow3 = Frame::new(
            self.basic_view.img_right_arrow3.x(),
            self.basic_view.img_right_arrow3.y() + y_offset,
            self.basic_view.img_right_arrow3.w(),
            self.basic_view.img_right_arrow3.h(),
            "🠖❯",
        );
        img_arrow3.set_frame(FrameType::FlatBox);
        img_arrow3.set_color(Color::by_index(53));
        img_arrow3.set_label_type(LabelType::Engraved);
        img_arrow3.set_label_font(Font::by_index(1));
        img_arrow3.set_label_size(15);
        img_arrow3.set_label_color(Color::by_index(229));
        local_tunnel_group.add(&img_arrow3);

        let mut img_firewall = Frame::new(
            self.basic_view.img_firewall.x(),
            self.basic_view.img_firewall.y() + y_offset,
            self.basic_view.img_firewall.w(),
            self.basic_view.img_firewall.h(),
            "firewall",
        );
        img_firewall.set_frame(FrameType::FlatBox);
        img_firewall.set_color(Color::by_index(92));
        img_firewall.set_label_type(LabelType::Embossed);
        img_firewall.set_label_color(Color::by_index(90));
        img_firewall.set_align(unsafe { std::mem::transmute(1) });
        local_tunnel_group.add(&img_firewall);

        let mut listen_addr_box = Frame::new(
            self.basic_view.local_listen_addr_box.x(),
            self.basic_view.local_listen_addr_box.y() + y_offset,
            self.basic_view.local_listen_addr_box.w(),
            self.basic_view.local_listen_addr_box.h(),
            "local listen addr",
        );
        listen_addr_box.set_frame(FrameType::UpBox);
        listen_addr_box.set_label_type(LabelType::Embossed);
        listen_addr_box.set_label_color(Color::by_index(229));
        listen_addr_box.set_align(unsafe { std::mem::transmute(1) });
        local_tunnel_group.add(&listen_addr_box);

        let mut ssh_server_box = Frame::new(
            self.basic_view.ssh_server_box.x(),
            self.basic_view.ssh_server_box.y() + y_offset,
            self.basic_view.ssh_server_box.w(),
            self.basic_view.ssh_server_box.h(),
            "SSH Server",
        );
        ssh_server_box.set_frame(FrameType::UpBox);
        ssh_server_box.set_label_type(LabelType::Embossed);
        ssh_server_box.set_label_color(Color::by_index(229));
        ssh_server_box.set_align(unsafe { std::mem::transmute(1) });
        local_tunnel_group.add(&ssh_server_box);

        let mut real_service_addr_box = Frame::new(
            self.basic_view.remote_addr_box.x(),
            self.basic_view.remote_addr_box.y() + y_offset,
            self.basic_view.remote_addr_box.w(),
            self.basic_view.remote_addr_box.h(),
            "remote service addr",
        );
        real_service_addr_box.set_frame(FrameType::UpBox);
        real_service_addr_box.set_label_type(LabelType::Embossed);
        real_service_addr_box.set_label_color(Color::by_index(229));
        real_service_addr_box.set_align(unsafe { std::mem::transmute(1) });
        local_tunnel_group.add(&real_service_addr_box);

        let listen_port_input = Input::new(
            self.basic_view.forward_port_input.x(),
            self.basic_view.forward_port_input.y() + y_offset,
            self.basic_view.forward_port_input.w(),
            self.basic_view.forward_port_input.h(),
            "0.0.0.0:",
        );
        local_tunnel_group.add(&listen_port_input);

        let ssh_server_host_input = Input::new(
            self.basic_view.ssh_server_ip_input.x(),
            self.basic_view.ssh_server_ip_input.y() + y_offset,
            self.basic_view.ssh_server_ip_input.w(),
            self.basic_view.ssh_server_ip_input.h(),
            "host:",
        );
        local_tunnel_group.add(&ssh_server_host_input);

        let ssh_server_port_input = Input::new(
            self.basic_view.ssh_port_input.x(),
            self.basic_view.ssh_port_input.y() + y_offset,
            self.basic_view.ssh_port_input.w(),
            self.basic_view.ssh_port_input.h(),
            "port:",
        );
        local_tunnel_group.add(&ssh_server_port_input);

        let ssh_server_username_input = Input::new(
            self.basic_view.ssh_username_input.x(),
            self.basic_view.ssh_username_input.y() + y_offset,
            self.basic_view.ssh_username_input.w(),
            self.basic_view.ssh_username_input.h(),
            "user:",
        );
        local_tunnel_group.add(&ssh_server_username_input);

        let ssh_server_pwd_input = Input::new(
            self.basic_view.ssh_pwd_input.x(),
            self.basic_view.ssh_pwd_input.y() + y_offset,
            self.basic_view.ssh_pwd_input.w(),
            self.basic_view.ssh_pwd_input.h(),
            "pwd:",
        );
        local_tunnel_group.add(&ssh_server_pwd_input);

        let real_service_host_input = Input::new(
            self.basic_view.remote_host_input.x(),
            self.basic_view.remote_host_input.y() + y_offset,
            self.basic_view.remote_host_input.w(),
            self.basic_view.remote_host_input.h(),
            "",
        );
        local_tunnel_group.add(&real_service_host_input);
        let real_service_port_input = Input::new(
            self.basic_view.remote_port_input.x(),
            self.basic_view.remote_port_input.y() + y_offset,
            self.basic_view.remote_port_input.w(),
            self.basic_view.remote_port_input.h(),
            ":",
        );
        local_tunnel_group.add(&real_service_port_input);

        let row = SSHTunnelRow {
            id,
            tunnel_type: TunnelType::Local,
            row_group: local_tunnel_group,
            index_txt,
            name_iuput,
            forward_type_choice,
            start_btn,
            stop_btn,
            del_btn,
            img_gray_line,
            img_pc,
            img_arrow1,
            img_arrow2,
            img_arrow3,
            img_firewall,
            listen_addr_box,
            ssh_server_box,
            real_service_addr_box,
            listen_port_input,
            ssh_server_host_input,
            ssh_server_port_input,
            ssh_server_username_input,
            ssh_server_pwd_input,
            real_service_host_input,
            real_service_port_input,
        };

        self.tunnel_rows.push(row);
    }
    // pub fn add_ssh_tunnel_row(&mut self) {
    //     let y = if self.tunnel_rows.len() > 0 {
    //         self.tunnel_rows.last().unwrap().2.y() + 45
    //     } else {
    //         self.basic_view.name_iuput.y()
    //     };

    //     let group_id = chrono::Local::now().timestamp_micros().to_string();

    //     let mut tunnel = Group::new(
    //         self.basic_view.tunnel_row.x(),
    //         if self.tunnel_rows.len() > 0 {
    //             self.tunnel_rows.last().unwrap().0.y() + 45
    //         } else {
    //             self.basic_view.tunnel_row.y()
    //         },
    //         self.basic_view.tunnel_row.w(),
    //         self.basic_view.tunnel_row.h(),
    //         None,
    //     );
    //     tunnel.set_label(&group_id);
    //     tunnel.end();
    //     tunnel.set_label_color(self.basic_view.tunnel_row.label_color());
    //     tunnel.set_color(self.basic_view.tunnel_row.color());
    //     tunnel.set_align(unsafe { std::mem::transmute(0) });
    //     tunnel.set_frame(FrameType::BorderBox);
    //     local_tunnel_group.add(&tunnel);

    //     let mut index_txt = Frame::new(
    //         self.basic_view.index_txt.x(),
    //         y,
    //         self.basic_view.index_txt.w(),
    //         self.basic_view.index_txt.h(),
    //         "0",
    //     );
    //     index_txt.set_label(&format!("{}", self.tunnel_rows.len()));
    //     index_txt.set_color(Color::by_index(46));
    //     index_txt.set_label_font(Font::by_index(1));
    //     index_txt.set_label_color(Color::by_index(229));
    //     tunnel.add(&index_txt);

    //     let mut forward_type_choice = Choice::new(
    //         self.basic_view.forward_type_choice.x(),
    //         y,
    //         self.basic_view.forward_type_choice.w(),
    //         self.basic_view.forward_type_choice.h(),
    //         "menu",
    //     );
    //     forward_type_choice.set_label("");
    //     forward_type_choice.add_choice("Local");
    //     forward_type_choice.add_choice("Remote");
    //     forward_type_choice.end();
    //     tunnel.add(&forward_type_choice);

    //     let mut start_btn = Button::new(
    //         self.basic_view.start_btn.x(),
    //         y - 2,
    //         self.basic_view.start_btn.w(),
    //         self.basic_view.start_btn.h(),
    //         None,
    //     );
    //     start_btn.set_image(Some(
    //         SharedImage::load("asset\\play.png")
    //             .expect("Could not find image: ..\\asset\\play.png"),
    //     ));
    //     start_btn.set_deimage(Some(
    //         SharedImage::load("asset\\inactive_play.png")
    //             .expect("Could not find image: ..\\asset\\inactive_play.png"),
    //     ));
    //     start_btn.set_frame(FrameType::FlatBox);
    //     start_btn.set_color(Color::by_index(255));
    //     start_btn.set_align(unsafe { std::mem::transmute(16) });
    //     start_btn.set_tooltip("start this ssh tunnel.");
    //     tunnel.add(&start_btn);
    //     let mut stop_btn = Button::new(
    //         self.basic_view.stop_btn.x(),
    //         y - 2,
    //         self.basic_view.stop_btn.width(),
    //         self.basic_view.stop_btn.h(),
    //         None,
    //     );
    //     stop_btn.set_image(Some(
    //         SharedImage::load("asset\\stop.png")
    //             .expect("Could not find image: ..\\asset\\stop.png"),
    //     ));
    //     stop_btn.set_deimage(Some(
    //         SharedImage::load("asset\\inactive_stop.png")
    //             .expect("Could not find image: ..\\asset\\inactive_stop.png"),
    //     ));
    //     stop_btn.set_frame(FrameType::FlatBox);
    //     stop_btn.set_color(Color::by_index(255));
    //     stop_btn.set_tooltip("stop this ssh tunnel.");
    //     stop_btn.set_align(unsafe { std::mem::transmute(16) });
    //     stop_btn.deactivate();
    //     tunnel.add(&stop_btn);

    //     let mut del_btn = Button::new(
    //         self.basic_view.del_btn.x(),
    //         y - 2,
    //         self.basic_view.del_btn.width(),
    //         self.basic_view.del_btn.h(),
    //         None,
    //     );
    //     del_btn.set_image(Some(
    //         SharedImage::load("asset\\del.png").expect("Could not find image: asset\\del.png"),
    //     ));
    //     del_btn.set_deimage(Some(
    //         SharedImage::load("asset\\inactive_del.png")
    //             .expect("Could not find image: ..\\asset\\inactive_del.png"),
    //     ));
    //     del_btn.set_frame(FrameType::FlatBox);
    //     del_btn.set_color(Color::by_index(255));
    //     del_btn.set_align(unsafe { std::mem::transmute(16) });
    //     del_btn.activate();
    //     tunnel.add(&del_btn);

    //     let mut name_iuput = Input::new(
    //         self.basic_view.name_iuput.x(),
    //         y,
    //         self.basic_view.name_iuput.w(),
    //         self.basic_view.name_iuput.h(),
    //         None,
    //     );
    //     name_iuput.set_label_type(LabelType::None);
    //     tunnel.add(&name_iuput);
    //     let mut forward_port_iuput = IntInput::new(
    //         self.basic_view.forward_port_iuput.x(),
    //         y,
    //         self.basic_view.forward_port_iuput.w(),
    //         self.basic_view.forward_port_iuput.h(),
    //         None,
    //     );
    //     forward_port_iuput.set_label_type(LabelType::None);
    //     tunnel.add(&forward_port_iuput);
    //     let mut dst_server_port_input = Input::new(
    //         self.basic_view.dst_server_port_input.x(),
    //         y,
    //         self.basic_view.dst_server_port_input.w(),
    //         self.basic_view.dst_server_port_input.h(),
    //         None,
    //     );
    //     dst_server_port_input.set_label_type(LabelType::None);
    //     tunnel.add(&dst_server_port_input);
    //     let mut ssh_username_iuput = Input::new(
    //         self.basic_view.ssh_username_iuput.x(),
    //         y,
    //         self.basic_view.ssh_username_iuput.w(),
    //         self.basic_view.ssh_username_iuput.h(),
    //         None,
    //     );
    //     ssh_username_iuput.set_label_type(LabelType::None);
    //     tunnel.add(&ssh_username_iuput);
    //     let mut ssh_server_ip_iuput = Input::new(
    //         self.basic_view.ssh_server_ip_iuput.x(),
    //         y,
    //         self.basic_view.ssh_server_ip_iuput.w(),
    //         self.basic_view.ssh_server_ip_iuput.h(),
    //         None,
    //     );
    //     ssh_server_ip_iuput.set_label_type(LabelType::None);
    //     tunnel.add(&ssh_server_ip_iuput);
    //     let mut ssh_port_iuput = IntInput::new(
    //         self.basic_view.ssh_port_iuput.x(),
    //         y,
    //         self.basic_view.ssh_port_iuput.w(),
    //         self.basic_view.ssh_port_iuput.h(),
    //         None,
    //     );
    //     ssh_port_iuput.set_label_type(LabelType::None);
    //     ssh_port_iuput.set_value("22");
    //     tunnel.add(&ssh_port_iuput);
    //     let mut pwd_input = SecretInput::new(
    //         self.basic_view.pwd_input.x(),
    //         y,
    //         self.basic_view.pwd_input.w(),
    //         self.basic_view.pwd_input.h(),
    //         None,
    //     );
    //     pwd_input.set_label_type(LabelType::None);
    //     tunnel.add(&pwd_input);
    //     let mut fl2rust_widget_10 = Frame::new(self.basic_view.at_box.x(), y, 20, 20, "@@");
    //     tunnel.add(&fl2rust_widget_10);
    //     let mut fl2rust_widget_11 = Frame::new(self.basic_view.box2.x(), y, 10, 20, ":");
    //     fl2rust_widget_11.set_label_font(Font::by_index(1));
    //     tunnel.add(&fl2rust_widget_11);

    //     let g_id = group_id.clone();
    //     start_btn.set_callback(move |b| {
    //         Self::send(UiMessage {
    //             msg_type: MsgType::StartTunnel,
    //             msg: g_id.to_owned(),
    //         })
    //     });

    //     let g_id = group_id.clone();
    //     stop_btn.set_callback(move |_| {
    //         Self::send(UiMessage {
    //             msg_type: MsgType::StopTunnel,
    //             msg: g_id.to_owned(),
    //         })
    //     });

    //     let g_id = group_id.clone();
    //     del_btn.set_callback(move |_| {
    //         Self::send(UiMessage {
    //             msg_type: MsgType::DeleteTunnel,
    //             msg: g_id.to_owned(),
    //         })
    //     });

    //     self.tunnel_rows.push((
    //         tunnel,
    //         index_txt,
    //         name_iuput,
    //         forward_type_choice,
    //         start_btn,
    //         stop_btn,
    //         forward_port_iuput,
    //         dst_server_port_input,
    //         ssh_username_iuput,
    //         ssh_server_ip_iuput,
    //         ssh_port_iuput,
    //         pwd_input,
    //         del_btn,
    //     ));
    // }

    pub fn delete_ssh_tunnel_row(&mut self, index: usize) {
        // println!("index:{index}");
        // if self.tunnel_rows.len() > index {
        //     for i in (index + 1..self.tunnel_rows.len()).rev() {
        //         let (x, y) = (self.tunnel_rows[i - 1].0.x(), self.tunnel_rows[i - 1].0.y());
        //         self.tunnel_rows[i].0.set_pos(x, y);
        //         self.tunnel_rows[i]
        //             .1
        //             .set_label((i - 1).to_string().as_str());
        //     }
        //     self.basic_view
        //         .scroll_view
        //         .remove(&self.tunnel_rows[index].0);
        //     let tunnel = self.tunnel_rows.remove(index);
        //     drop(tunnel);

        //     self.basic_view.main_window.redraw();
        // }
    }

    // pub fn try_verify_start_tunnel_params(
    //     &self,
    //     index: usize,
    // ) -> Result<(String, String, i32, String, String, String, i32, String)> {
    //     let (
    //         _,
    //         _,
    //         name_iuput,
    //         forward_type_choice,
    //         _,
    //         _,
    //         forward_port_iuput,
    //         dst_server_port_input,
    //         ssh_username_iuput,
    //         ssh_server_ip_iuput,
    //         ssh_port_iuput,
    //         pwd_input,
    //         _,
    //     ): &(
    //         Group,
    //         Frame,
    //         Input,
    //         Choice,
    //         Button,
    //         Button,
    //         IntInput,
    //         Input,
    //         Input,
    //         Input,
    //         IntInput,
    //         SecretInput,
    //         Button,
    //     ) = &self.tunnel_rows.get(index).unwrap();
    //     let name = name_iuput.value();
    //     let forwart_type = match forward_type_choice.choice() {
    //         Some(c) => c,
    //         None => return Err(anyhow!("[Type] must be select.")),
    //     };
    //     let forward_port = forward_port_iuput.value().parse::<i32>()?;
    //     let dst_server_port = dst_server_port_input.value();
    //     if dst_server_port.is_empty() {
    //         return Err(anyhow!("[Dst host:port] can not be empty."));
    //     }

    //     let ssh_username = ssh_username_iuput.value();
    //     if ssh_username.is_empty() {
    //         return Err(anyhow!("[username] can not be empty."));
    //     }

    //     let ssh_server_ip = ssh_server_ip_iuput.value();
    //     if ssh_server_ip.is_empty() {
    //         return Err(anyhow!("[ssh server ip] can not be empty."));
    //     }
    //     let ssh_port = ssh_port_iuput.value().parse::<i32>()?;
    //     let pwd = pwd_input.value();
    //     if pwd.is_empty() {
    //         return Err(anyhow!("[password] can not be empty."));
    //     }

    //     Ok((
    //         name,
    //         forwart_type,
    //         forward_port,
    //         dst_server_port,
    //         ssh_username,
    //         ssh_server_ip,
    //         ssh_port,
    //         pwd,
    //     ))
    // }

    // pub fn get_cur_index(&self, key: &str) -> usize {
    //     let mut index = usize::MAX;
    //     for i in 0..self.tunnel_rows.len() {
    //         if self.tunnel_rows[i].0.label() == key.to_string() {
    //             index = i;
    //             break;
    //         }
    //     }
    //     println!("cur index:{index}");
    //     return index;
    // }
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

pub fn handle_view_msg(
    view: &mut SSHTunnelView,
    ui_msg: UiMessage,
    map: &mut HashMap<usize, SSHTunnel>,
) {
    match ui_msg.msg_type {
        MsgType::INFO => todo!(),
        MsgType::ERROR => todo!(),
        MsgType::AddLocalTunnelRow => view.add_ssh_remote_tunnel_row(),
        MsgType::AddRemoteTunnelRow => view.add_ssh_local_tunnel_row(),
        MsgType::StartTunnel => {
            // println!("recv message :{:?}", ui_msg);
            // let index = view.get_cur_index(&ui_msg.msg);

            // if index == usize::MAX {
            //     fltk::dialog::alert_default(&format!(
            //         "can not found this({}) tunnel row.",
            //         ui_msg.msg
            //     ));
            //     return;
            // }

            // let (
            //     name,
            //     forwart_type,
            //     forward_port,
            //     dst_server_port,
            //     ssh_username,
            //     ssh_server_ip,
            //     ssh_port,
            //     pwd,
            // ) = match view.try_verify_start_tunnel_params(index) {
            //     Ok(t) => t,
            //     Err(err) => {
            //         fltk::dialog::alert_default(&format!("Oops! An error occurred:{:?}", err));
            //         return;
            //     }
            // };

            // let (
            //     _,
            //     _,
            //     _,
            //     forward_type_choice,
            //     start_btn,
            //     stop_btn,
            //     forward_port_iuput,
            //     dst_server_port_input,
            //     ssh_username_iuput,
            //     ssh_server_ip_iuput,
            //     ssh_port_iuput,
            //     pwd_input,
            //     del_btn,
            // ): &mut (
            //     Group,
            //     Frame,
            //     Input,
            //     Choice,
            //     Button,
            //     Button,
            //     IntInput,
            //     Input,
            //     Input,
            //     Input,
            //     IntInput,
            //     SecretInput,
            //     Button,
            // ) = view.tunnel_rows.get_mut(index).unwrap();

            // start_btn.deactivate();
            // stop_btn.deactivate();

            // if map.contains_key(&index) {
            //     let st = map.remove(&index).unwrap();
            //     drop(st);
            // }

            // let mut ssh_tunnel = SSHTunnel::new(
            //     &ui_msg.msg,
            //     &name,
            //     &forwart_type,
            //     forward_port,
            //     &dst_server_port,
            //     &ssh_username,
            //     &ssh_server_ip,
            //     ssh_port,
            //     &pwd,
            //     crate::ssh_tunnel::Status::Started,
            // );

            // ssh_tunnel.start_tunnel().unwrap();
            // map.insert(index, ssh_tunnel);

            // stop_btn.activate();
            // del_btn.deactivate();
            // forward_type_choice.deactivate();
            // forward_port_iuput.deactivate();
            // dst_server_port_input.deactivate();
            // ssh_username_iuput.deactivate();
            // ssh_server_ip_iuput.deactivate();
            // ssh_port_iuput.deactivate();
            // pwd_input.deactivate();
        }
        MsgType::StopTunnel => {
            // println!("recv message :{:?}", ui_msg);
            // let index = view.get_cur_index(&ui_msg.msg);

            // if index == usize::MAX {
            //     fltk::dialog::alert_default(&format!(
            //         "can not found this({}) tunnel row.",
            //         ui_msg.msg
            //     ));
            //     return;
            // }
            // let tunnel = map.get_mut(&index).unwrap();
            // tunnel.stop_tunnel();

            // let (
            //     _,
            //     _,
            //     _,
            //     forward_type_choice,
            //     start_btn,
            //     stop_btn,
            //     forward_port_iuput,
            //     dst_server_port_input,
            //     ssh_username_iuput,
            //     ssh_server_ip_iuput,
            //     ssh_port_iuput,
            //     pwd_input,
            //     del_btn,
            // ): &mut (
            //     Group,
            //     Frame,
            //     Input,
            //     Choice,
            //     Button,
            //     Button,
            //     IntInput,
            //     Input,
            //     Input,
            //     Input,
            //     IntInput,
            //     SecretInput,
            //     Button,
            // ) = view.tunnel_rows.get_mut(index).unwrap();

            // stop_btn.deactivate();
            // start_btn.activate();
            // del_btn.activate();
            // forward_type_choice.activate();
            // forward_port_iuput.activate();
            // dst_server_port_input.activate();
            // ssh_username_iuput.activate();
            // ssh_server_ip_iuput.activate();
            // ssh_port_iuput.activate();
            // pwd_input.activate();
        }
        MsgType::ResizeMainWindow => {
            let arr = ui_msg.msg.split("|").collect::<Vec<&str>>();
            let w = arr.get(2).unwrap().parse::<i32>().unwrap();
            let h = arr.get(3).unwrap().parse::<i32>().unwrap();

            view.basic_view.scroll_view.resize(
                view.basic_view.scroll_view.x(),
                view.basic_view.scroll_view.y(),
                w,
                h - 60,
            );
            view.basic_view.main_window.redraw();
        }
        MsgType::DeleteTunnel => {
            // let index = view.get_cur_index(&ui_msg.msg);

            // if index == usize::MAX {
            //     fltk::dialog::alert_default(&format!(
            //         "can not found this({}) tunnel row.",
            //         ui_msg.msg
            //     ));
            //     return;
            // }
            // if map.contains_key(&index) {
            //     let st = map.remove(&index).unwrap();
            //     drop(st);
            // }
            // view.delete_ssh_tunnel_row(index);
        }
    }

    view.basic_view.scroll_view.redraw();
}
