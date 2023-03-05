mod basic_view;
mod ssh_tunnel;
mod ssh_tunnel_view;

use std::borrow::BorrowMut;
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
use ssh_tunnel::{SSHTunnel, RWLOCK_SSH_TUNNEL_MAP};
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
        .remove(&view.basic_view.tunnel_g1);
    view.add_ssh_tunnel_row();

    app::add_idle3(move |_| match RWLOCK_MSG_CHANNEL.read() {
        Err(err) => fltk::dialog::alert_default(&format!("RWLOCK_MSG_CHANNEL error:{:?}", err)),
        Ok(r) => {
            let ui_msg = r.1.recv();
            if let Some(m) = ui_msg {
                println!("msg:{:?}", m);
                handle_msg(&mut view, m);
            }
        }
    });

    app.run().unwrap();
}

fn handle_msg(view: &mut SSHTunnelView, ui_msg: UiMessage) {
    match ui_msg.msg_type {
        MsgType::INFO => todo!(),
        MsgType::ERROR => todo!(),
        MsgType::AddTunnelRow => view.add_ssh_tunnel_row(),
        MsgType::StartTunnel => {
            println!("recv message :{:?}", ui_msg);
            let index: usize = ui_msg.msg.parse().unwrap();
            let (
                g,
                name,
                type_input,
                forward_port,
                dst_host_port,
                ssh_user_server_port,
                start_btn,
                stop_btn,
            ) = &view.tunnel_rows.get_mut(index).unwrap();
            let key = &ui_msg.msg;

            // start_btn.deactivate();
            // stop_btn.activate();

            loop {
                match RWLOCK_SSH_TUNNEL_MAP.try_write() {
                    Ok(mut map) => {
                        if map.contains_key(key) {
                            map.remove(key);
                        }
                        let mut ssh_tunnel = SSHTunnel::new(
                            &ui_msg.msg,
                            &name.value().clone(),
                            &forward_port.value().clone(),
                            &dst_host_port.value().clone(),
                            &ssh_user_server_port.value().clone(),
                        );
                        ssh_tunnel.start_tunnel().unwrap();
                        map.insert(key.to_string(), ssh_tunnel);
                        //std::thread::sleep(Duration::from_secs(30));
                        break;
                    }
                    Err(err) => {
                        println!("can not write RWLOCK_SSH_TUNNEL_MAP, error:{:?}, will be sleep 10 sec.",err);
                        std::thread::sleep(Duration::from_secs(10));
                    }
                }
            }
        }
        MsgType::StopTunnel => todo!(),
    }

    view.basic_view.scroll_view.redraw();
}
