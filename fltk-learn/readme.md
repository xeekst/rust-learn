#### FLTK 
### 一、ui设计器 - fluid
1. 大体设计可以使用 fluid 这个ui设计器进行简单的设计
2. 再通过 fl2rust 来转换 .fl 文件为 .rs 文件
```
fl2rust view.fl > view.rs
```
3. 也可定义build.rs
- 引入 Cargo.toml
```toml
[build-dependencies]
fl2rust = "0.4"
```
- 定义build.rs 
```rust
fn main() {
    // use std::env;
    // use std::path::PathBuf;
    // 以下代码告诉 Cargo ，一旦指定的文件 `src/register/register_view.fl` 发生了改变，就重新运行当前的构建脚本
    println!("cargo:rerun-if-changed=src/view.fl");
    let g = fl2rust::Generator::default();
    //let out_path = PathBuf::from("src/register/register_view.rs");
    g.in_out("src/view.fl", "src/view.rs")
        .expect("Failed to generate rust from fl file!");
}

```

### 二、ui设计 —— 纯手敲
1. 可以使用 flex 布局
```rust
use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    let mut flex = Flex::new(0, 0, 400, 300, None);
    flex.set_type(group::FlexType::Column);
    let expanding = button::Button::default().with_label("Expanding");
    let mut normal = button::Button::default().with_label("Normal");
    flex.set_size(&mut normal, 30);
    flex.end();
    win.end();
    win.show();
    a.run().unwrap();
}

```
2. Grid 布局
```rust
use fltk::{prelude::*, *};
use fltk_grid::Grid;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(500, 300);
    let mut grid = Grid::default_fill();
    // 设置为 "true "以显示单元格的框线和数字
    grid.debug(false); 
    // 5 行，5 列
    grid.set_layout(5, 5); 
    // 组件，行，列
    grid.insert(&mut button::Button::default().with_label("Click"), 0, 1); 
    // widget, row, col, row_span, col_span
    grid.insert_ext(&mut button::Button::default().with_label("Button 2"), 2, 1, 3, 1); 
    win.end();
    win.show();
    a.run().unwrap();
}

```

### 三、 UI widget 及数据 相互之间的交互 —— 自定义事件且需要关心的 widget 监听
> 优点 是更加清晰逻辑，可以定义MVC架构
```rust
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
```

### 四、自定义 UIMessage 且 在app.idle3() 中处理所有的交互
> 优点是所有权方便管控，统一管理
```rust
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


fn main() {
    main_use_ui_message();
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

```

### 五、也可使用 webview （依赖webview的库）
1. 引入webview Cargo.toml
```toml
[dependencies]
fltk = "1"
fltk-webview = "0.2"
```

2. 代码
```rust
use fltk::{app, prelude::*, window};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.make_resizable(true);
    win.show();

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("https://baidu.com");
    
    app.run().unwrap();
}
```