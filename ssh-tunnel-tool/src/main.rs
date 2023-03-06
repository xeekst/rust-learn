mod basic_view;
mod ssh_tunnel;
mod ssh_tunnel_view;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Stdin, Write};
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use fltk::app::{self, Receiver, Sender};
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
use ssh_tunnel::{check_ssh_tunnels, SSHTunnel};
use ssh_tunnel_view::SSHTunnelView;

lazy_static::lazy_static! {
    pub static ref RWLOCK_MSG_CHANNEL: RwLock<(Sender<UiMessage>, Receiver<UiMessage>)> =
        RwLock::new(fltk::app::channel::<UiMessage>());
}
#[derive(Debug)]
pub struct UiMessage {
    pub msg_type: MsgType,
    pub msg: String,
}

#[derive(Debug, PartialEq)]
pub enum MsgType {
    INFO,
    ERROR,

    AddTunnelRow,
    StartTunnel,
    StopTunnel,
}

fn main() {
    let app = app::App::default();
    let mut view = SSHTunnelView::new();
    view.init();
    view.basic_view
        .scroll_view
        .remove(&view.basic_view.tunnel_row);
    view.add_ssh_tunnel_row();

    let mut map: HashMap<String, SSHTunnel> = HashMap::new();
    let mut last_time = chrono::Local::now();

    app::add_idle3(move |_| match RWLOCK_MSG_CHANNEL.read() {
        Err(err) => fltk::dialog::alert_default(&format!("RWLOCK_MSG_CHANNEL error:{:?}", err)),
        Ok(r) => {
            let ui_msg = r.1.recv();
            if let Some(m) = ui_msg {
                println!("msg:{:?}", m);
                handle_msg(&mut view, m, &mut map);
            }

            if (chrono::Local::now() - last_time) > chrono::Duration::seconds(10) {
                last_time = chrono::Local::now();
                check_ssh_tunnels(&mut map);
            }
        }
    });

    app.run().unwrap();
}

fn handle_msg(view: &mut SSHTunnelView, ui_msg: UiMessage, map: &mut HashMap<String, SSHTunnel>) {
    match ui_msg.msg_type {
        MsgType::INFO => todo!(),
        MsgType::ERROR => todo!(),
        MsgType::AddTunnelRow => view.add_ssh_tunnel_row(),
        MsgType::StartTunnel => {
            println!("recv message :{:?}", ui_msg);
            let index: usize = ui_msg.msg.parse().unwrap();
            let (
                tunnel,
                check_box,
                index_txt,
                name_iuput,
                forward_type_choice,
                ref mut start_btn,
                ref mut stop_btn,
                forward_port_iuput,
                dst_server_port_input,
                ssh_username_iuput,
                ssh_server_ip_iuput,
                ssh_port_iuput,
                pwd_input,
            ): &mut (
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
            ) = view.tunnel_rows.get_mut(index).unwrap();
            let key = &ui_msg.msg;

            start_btn.deactivate();
            stop_btn.deactivate();

            if map.contains_key(key) {
                map.remove(key);
            }
            let mut ssh_tunnel = SSHTunnel::new(
                &ui_msg.msg,
                &name_iuput.value().clone(),
                &forward_port_iuput.value().to_string(),
                &dst_server_port_input.value().clone(),
                &ssh_username_iuput.value().clone(),
                &ssh_server_ip_iuput.value().clone(),
                &ssh_port_iuput.value().to_string(),
                &pwd_input.value().clone(),
            );
            ssh_tunnel.start_tunnel().unwrap();
            map.insert(key.to_string(), ssh_tunnel);

            stop_btn.activate();
        }
        MsgType::StopTunnel => todo!(),
    }

    view.basic_view.scroll_view.redraw();
}
