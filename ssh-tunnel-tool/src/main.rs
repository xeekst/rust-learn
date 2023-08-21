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
use fltk_theme::{WidgetTheme, ThemeType};
use ssh_tunnel::{check_ssh_tunnels, SSHTunnel};
use ssh_tunnel_view::SSHTunnelView;

use crate::ssh_tunnel_view::handle_view_msg;

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
    ResizeMainWindow,
    StartTunnel,
    StopTunnel,
    DeleteTunnel,
}

fn main() {
    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();
    let mut view = SSHTunnelView::new();
    view.init();
    // view.basic_view
    //     .scroll_view
    //     .remove(&view.basic_view.tunnel_row);
    // view.add_ssh_tunnel_row();

    let mut map: HashMap<usize, SSHTunnel> = HashMap::new();
    let mut last_time = chrono::Local::now();

    app::add_idle3(move |_| match RWLOCK_MSG_CHANNEL.read() {
        Err(err) => fltk::dialog::alert_default(&format!("RWLOCK_MSG_CHANNEL error:{:?}", err)),
        Ok(r) => {
            let ui_msg = r.1.recv();
            if let Some(m) = ui_msg {
                println!("msg:{:?}", m);
                handle_view_msg(&mut view, m, &mut map);
            }
            else {
                thread::sleep(Duration::from_millis(1));
            }
        
            if (chrono::Local::now() - last_time) > chrono::Duration::seconds(5) {
                last_time = chrono::Local::now();
                check_ssh_tunnels(&mut map);
            }
        }
    });

    app.run().unwrap();
}
