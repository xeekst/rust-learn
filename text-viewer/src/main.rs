use std::sync::{atomic::AtomicI32, Arc, Mutex, RwLock};
use std::time::Instant;

use fltk::enums::{Color, Event};
use fltk::prelude::GroupExt;
use fltk::text;
use fltk::{
    app,
    prelude::{DisplayExt, ValuatorExt, WidgetBase, WidgetExt},
    text::TextDisplay,
    valuator::Scrollbar,
};
use fltk_richtext::{RichTextBuilder, RichTextDisplay, Style};
use fltk_theme::{ThemeType, WidgetTheme};

lazy_static::lazy_static! {
    pub static ref SCROLL_BAR_POS: AtomicI32 = AtomicI32::new(0);
}

trait RichTextLog {
    fn init(&mut self);
    fn append_highlight(&mut self, s: &str);
    fn info(&mut self, s: &str, highlight: Option<&str>);
    fn error(&mut self, s: &str, highlight: Option<&str>);
    fn warn(&mut self, s: &str, highlight: Option<&str>);
    fn debug(&mut self, s: &str, highlight: Option<&str>);
    fn default(&mut self, s: &str, highlight: Option<&str>);
    fn write(&mut self, s: &str, font_color: Color, highlight: Option<&str>);
}

impl RichTextLog for RichTextBuilder {
    fn write(&mut self, s: &str, font_color: Color, highlight: Option<&str>) {
        match highlight {
            Some(h) => {
                let vec = s.split(h).collect::<Vec<&str>>();
                if s.starts_with(h) {
                    self.append_highlight(h);
                }
                for (index, txt) in vec.iter().enumerate() {
                    self.append(
                        txt,
                        Style {
                            color: font_color,
                            ..Default::default()
                        },
                    );
                    if index != vec.len() - 1 {
                        self.append_highlight(h);
                    }
                }
                if s.ends_with(h) {
                    self.append_highlight(h);
                }
            }
            None => self.append(
                s,
                Style {
                    color: font_color,
                    ..Default::default()
                },
            ),
        }
    }

    fn info(&mut self, s: &str, highlight: Option<&str>) {
        self.write(s, Color::from_rgb(10, 163, 68), highlight);
    }

    fn error(&mut self, s: &str, highlight: Option<&str>) {
        self.write(s, Color::from_rgb(244, 121, 131), highlight);
    }

    fn warn(&mut self, s: &str, highlight: Option<&str>) {
        self.write(s, Color::from_rgb(250, 140, 53), highlight);
    }

    fn debug(&mut self, s: &str, highlight: Option<&str>) {
        self.write(s, Color::from_rgb(23, 124, 176), highlight);
    }

    fn default(&mut self, s: &str, highlight: Option<&str>) {
        self.write(s, Color::from_rgb(214, 236, 240), highlight);
    }

    fn append_highlight(&mut self, s: &str) {
        self.append(
            s,
            Style {
                color: Color::White,
                bgcolor: Color::from_rgb(255, 51, 0),
                attr: text::TextAttr::BgColor,
                ..Default::default()
            },
        );
    }

    fn init(&mut self) {
        self.append_highlight("");
        self.default("", None);
        self.error("", None);
        self.warn("", None);
        self.debug("", None);
        self.info("", None);
        self.default("", None);
    }
}

#[derive(Clone)]
struct CustomTextDisplay {
    page_size: i32,
    page_index: i32,
    line_count_one_page: i32,
    line_height: i32,
    page_total: i32,
    data_pages: Arc<RwLock<Vec<RichTextBuilder>>>,
    text_display: TextDisplay,
    scrollbar: Scrollbar,
}

impl CustomTextDisplay {
    fn new(mut text_display: TextDisplay) -> Self {
        let page_size = 100000;
        let line_count_one_page = 50;
        let line_height = 3;
        //text_display.set_text_size(10);
        //text_display.height() / text_display.text_size();
       //println!("font size:{:?}",text_display.measure_font());

        let mut scrollbar = Scrollbar::new(text_display.x() + text_display.width() - 13, text_display.y(), 15, text_display.height(), "");
        scrollbar.set_slider_size(0.05);
        let data_pages: Arc<RwLock<Vec<RichTextBuilder>>> = Arc::new(RwLock::new(vec![RichTextBuilder::new()]));

        let data_pages_clone2 = data_pages.clone();
        let mut scrollbar_clone = scrollbar.clone();
        let rich_vec = data_pages_clone2.write().unwrap();
        let mut rich = rich_vec[0].clone();
        rich.init();
        //text_display.s
        text_display.set_rich_text(rich);
        text_display.handle(move |s, event| {
            match event {
                Event::KeyDown => {
                    let key = app::event_key();
                    
                    if key == fltk::enums::Key::PageUp {
                        // 执行 Page Up 滚动操作
                        // text_display.scroll(text_display.scrollbar_yvalue() - text_display.scrollbar_size());
                    } else if key == fltk::enums::Key::PageDown {
                        // 执行 Page Down 滚动操作
                       // text_display.scroll(text_display.scrollbar_yvalue() + text_display.scrollbar_size());
                    }

                    true
                }
                Event::MouseWheel => {
                    let dy = app::event_dy();

                    let pos = scrollbar_clone.value();
                    let r = match dy {
                        app::MouseWheel::Up => {
                            if pos + line_height as f64 > scrollbar_clone.maximum() - line_count_one_page as f64 {
                                scrollbar_clone.set_value(scrollbar_clone.maximum() - line_count_one_page as f64);
                            } else {
                                scrollbar_clone.set_value(pos + line_height as f64);
                            }
                            //scrollbar_clone.do_callback();
                            println!("Scroll Position: {}", pos + line_height as f64);
                            println!("Mouse wheel up");
                            //scrollbar_clone.handle_event(Event:)
                            true
                        }
                        app::MouseWheel::Down => {
                            if (pos - line_height as f64) < 0.0 {
                                scrollbar_clone.set_value(0.0);
                            } else {
                                scrollbar_clone.set_value(pos - line_height as f64);
                            }
                            //scrollbar_clone.do_callback();
                            println!("Scroll Position: {}", pos - line_height as f64);
                            true
                        }
                        _ => false,
                    };
                    //scrollbar.set_value(SCROLL_BAR_POS.load(std::sync::atomic::Ordering::Relaxed) as f64);
                    return r; // 处理事件
                }
                _ => false, // 不处理其他事件
            }
        });
        let mut td_clone = text_display.clone();
        let data_pages_clone = data_pages.clone();

        let ctd = CustomTextDisplay {
            text_display,
            scrollbar: scrollbar.clone(),
            page_size,
            page_index: 0,
            page_total: 0,
            line_count_one_page,
            line_height,
            data_pages,
        };
        let mut ctd_clone = ctd.clone();
        scrollbar.set_callback(move |s| {
            let pos = s.value();
            let page_offset = ((pos - (ctd_clone.page_index * page_size) as f64) / page_size as f64).abs().ceil() as i32;
            if pos > ((ctd_clone.page_index + 1) * page_size - line_count_one_page).into() {
                println!("more");
                let datas = match data_pages_clone.try_read() {
                    Ok(d) => d,
                    Err(err) => {
                        println!("data is waiting write.{:?}", err);
                        return;
                    }
                };
                ctd_clone.page_index += page_offset;

                if ctd_clone.page_index >= datas.len() as i32 {
                    ctd_clone.page_index = (datas.len() - 1) as i32;
                }
                let rich_page = datas[ctd_clone.page_index as usize].clone();
                td_clone.set_rich_text(rich_page);
            } else if pos < (ctd_clone.page_index * page_size ).into() {
                println!("less");
                let datas = match data_pages_clone.try_read() {
                    Ok(d) => d,
                    Err(err) => {
                        println!("data is waiting write.{:?}", err);
                        return;
                    }
                };
                ctd_clone.page_index -= page_offset;
                let rich_page = datas[ctd_clone.page_index as usize].clone();
                td_clone.set_rich_text(rich_page);
            }
            println!("scroll {}",pos as i32 - (ctd_clone.page_index * page_size));
            td_clone.scroll(pos as i32 - (ctd_clone.page_index * page_size), 0);
        });

        ctd
    }

    fn insert(&mut self, lines: Vec<&str>) {
        // 处理滚动条拖动事件，这里只是打印滚动条位置
        //println!("insert data: {}", line);

        let mut rich_vec = self.data_pages.write().unwrap();
        for line in lines {
            if rich_vec.len() == 0 || (self.page_total != 0 && self.page_total % self.page_size == 0) {
                let mut rich = RichTextBuilder::new();
                rich.init();
                rich_vec.push(rich);
            }
            self.page_total += 1;
            let rich_text_buf = rich_vec.last_mut().unwrap();

            match line {
                s if s.contains("ERROR") => rich_text_buf.error(&format!("{}\n", line), None),
                s if s.contains("WARN") => rich_text_buf.warn(&format!("{}\n", line), None),
                s if s.contains("INFO") => rich_text_buf.info(&format!("{}\n", line), None),
                s if s.contains("DEBUG") => rich_text_buf.debug(&format!("{}\n", line), None),
                _ => rich_text_buf.default(&format!("{}\n", line), None),
            };
            self.scrollbar.set_maximum((self.page_total + self.line_count_one_page) as f64);
        }
    }

    fn find(&mut self, txt: &str) {}
}

fn main() {
    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();
    let mut win = fltk::window::Window::new(100, 100, 830, 900, "TextDisplay Scrollbar Event Example");
    let mut text_display = TextDisplay::new(10, 10, 780, 840, "");
    text_display.set_scrollbar_size(10); // 设置滚动条大小为零
    text_display.set_linenumber_width(50);
    let mut td = CustomTextDisplay::new(text_display);

    std::thread::spawn(move || {
        let start = Instant::now();
        let content = std::fs::read_to_string("log.log").unwrap();
        let contests = content.split("\n").collect::<Vec<&str>>();
        println!("insert");
        td.insert(contests);
        // 记录结束时间
        let end = Instant::now();
        // 计算时间间隔
        let duration = end - start;
        // 输出执行时间（单位：纳秒）
        println!("Execution time: {} as_millis", duration.as_millis());
    });

    win.end();
    win.show();

    app.run().unwrap();
}
