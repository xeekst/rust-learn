use fltk::{
    app::{self, Receiver, Sender},
    button::*,
    enums::*,
    frame::*,
    group::*,
    prelude::*,
    window::*,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;

pub struct MyEvent;

impl MyEvent {
    const CHANGED: i32 = 40;
}

#[derive(Clone)]
pub struct Counter {
    count: Rc<RefCell<i32>>,
}

impl Counter {
    pub fn new(val: i32) -> Self {
        Counter {
            count: Rc::from(RefCell::from(val)),
        }
    }

    pub fn increment(&mut self) {
        *self.count.borrow_mut() += 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn decrement(&mut self) {
        *self.count.borrow_mut() -= 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn value(&self) -> i32 {
        *self.count.borrow()
    }
}

fn main_use_widget_handle() {
    let app = app::App::default();
    let counter = Counter::new(0);
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_inc = Button::default().with_size(0, 40).with_label("+");
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label(&counter.clone().value().to_string());
    let mut but_dec = Button::default().with_size(0, 40).with_label("-");
    pack.end();
    wind.end();
    wind.show();

    but_inc.set_callback({
        let mut c = counter.clone();
        move |_| c.increment()
    });

    but_dec.set_callback({
        let mut c = counter.clone();
        move |_| c.decrement()
    });
    frame.handle(move |f, ev| {
        println!("event:{}", ev);
        let la = &wind.label();
        if ev == MyEvent::CHANGED.into() {
            f.set_label(&format!("{},{}", la, &counter.clone().value().to_string()));
            true
        } else {
            false
        }
    });

    app.run().unwrap();
}

fn main() {
    main_use_widget_handle();
}

lazy_static! {
    pub static ref RWLOCK_MSG_CHANNEL: RwLock<(Sender<UiMessage>, Receiver<UiMessage>)> =
        RwLock::new(fltk::app::channel::<UiMessage>());
}
pub struct UiMessage {
    pub msg_type: MsgType,
    pub msg: String,
}

#[derive(PartialEq)]
pub enum MsgType {
    ADD,
    SUB,
}

fn send_event(event: UiMessage) {
    match &RWLOCK_MSG_CHANNEL.read() {
        Ok(channel) => {
            let sender = &channel.0;
            sender.send(event);
        }
        Err(err) => panic!("get message channel sender error:{}", err),
    }
}

#[derive(Clone)]
pub struct MSGCounter {
    count: Rc<RefCell<i32>>,
}

impl MSGCounter {
    pub fn new(val: i32) -> Self {
        MSGCounter {
            count: Rc::from(RefCell::from(val)),
        }
    }

    pub fn increment(&mut self) {
        *self.count.borrow_mut() += 1;
        send_event(UiMessage {
            msg_type: MsgType::ADD,
            msg: String::from(format!("{}", self.count.borrow())),
        })
    }

    pub fn decrement(&mut self) {
        *self.count.borrow_mut() -= 1;
        send_event(UiMessage {
            msg_type: MsgType::SUB,
            msg: String::from(format!("{}", self.count.borrow())),
        })
    }

    pub fn value(&self) -> i32 {
        *self.count.borrow()
    }
}

fn main_use_ui_message() {
    let app = app::App::default();
    let counter = MSGCounter::new(0);
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_inc = Button::default().with_size(0, 40).with_label("+");
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label(&counter.clone().value().to_string());
    let mut but_dec = Button::default().with_size(0, 40).with_label("-");
    pack.end();
    wind.end();
    wind.show();

    but_inc.set_callback({
        let mut c = counter.clone();
        move |_| c.increment()
    });

    but_dec.set_callback({
        let mut c = counter.clone();
        move |_| c.decrement()
    });
    app::add_idle3(move |_| {
        let r = RWLOCK_MSG_CHANNEL.read().unwrap();
        let ui_msg = r.1.recv();
        if let Some(m) = ui_msg {
            match m.msg_type {
                MsgType::ADD => frame.set_label(&format!("ADD UIMessage:{}", m.msg)),
                MsgType::SUB => frame.set_label(&format!("SUB UIMessage:{}", m.msg)),
            }
        }
    });

    app.run().unwrap();
}
