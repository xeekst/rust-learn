use std::{sync::atomic::AtomicI32, time::Instant, io::{Write, BufWriter}, fs};

use fltk::{
    app,
    enums::{Align, Event, Key},
    prelude::*,
    text::{self, TextDisplay},
};
use fltk_richtext::{RichTextBuilder, RichTextDisplay};
use fltk_theme::{ThemeType, WidgetTheme};

lazy_static::lazy_static! {
    pub static ref SCROLL_BAR_POS: AtomicI32 = AtomicI32::new(0);
    pub static ref PAGE_INDEX: AtomicI32 = AtomicI32::new(0);
}
struct CustomTextDisplay {
    text_display: TextDisplay,
}

impl CustomTextDisplay {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let text_display = TextDisplay::new(x, y, width, height, "");
        CustomTextDisplay { text_display }
    }

    fn handle_scroll(&self, pos: f64) {
        // 处理滚动条拖动事件，这里只是打印滚动条位置
        println!("Scrollbar position: {}", pos);
    }
}

// impl app::Handler for CustomTextDisplay {
//     fn handle(&mut self, event: Event) -> bool {
//         match event {
//             Event::Scroll => {
//                 let scrollbar_pos = self.text_display.scrollbar().unwrap().value();
//                 self.handle_scroll(scrollbar_pos);
//                 true
//             }
//             _ => false,
//         }
//     }
// }

struct TextDisplayBuffer {
    current: usize,
    window_size: usize,
    lines: Vec<String>,
}

impl TextDisplayBuffer {
    fn refill(&mut self) {}
}

impl Iterator for TextDisplayBuffer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.lines.len() {
            self.current += 1;

            Some(self.lines[self.current].clone())
        } else {
            None
        }
    }
}

fn main() {
    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();
    let mut win =
        fltk::window::Window::new(100, 100, 430, 600, "TextDisplay Scrollbar Event Example");
    let mut text_display = TextDisplay::new(10, 10, 380, 580, "");
    //text_display.wrap_mode(text::WrapMode::AtColumn, 0);
    text_display.set_scrollbar_size(10); // 设置滚动条大小为零
    text_display.set_linenumber_width(50);

    //win.scrollbar();
    let count = 155500;
    let window_size: i32 = 5000;
    let td_h: i32 = 31;
    let line_h = 3;
    let window_pre_offset: usize = 0;
    let mut line_buffer = vec![];
    
    //let mut contents= "".to_string();
    for i in 0..count {
        let text = format!("{i} Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line Line {i}\n");
        //contents += text.as_str();
        line_buffer.push(text);
    }
    //fs::write("t.txt",contents).unwrap();
    let element_size = std::mem::size_of::<String>();  // 单个元素（String）的大小
    let elements_size: usize = line_buffer.iter().map(|s| s.capacity()).sum();  // 所有元素中字符串的大小

    let total_size = std::mem::size_of::<Vec<String>>() + (element_size * line_buffer.len()) + elements_size;  // 包含所有元素的总大小

    println!("line_buffer:{} mb",total_size / 1024 / 1024);
    let mut td_pages = vec![];
    let mut l = 0;
    for k in 0..count/window_size +1 {
        let mut rich_text_buf = RichTextBuilder::new();
        for i in l..(window_size as usize+l)  {
            if i < line_buffer.len() {
                rich_text_buf.append(&line_buffer[i], None);
            }
           
        }
        td_pages.push(rich_text_buf);
        l = ((k +1) * window_size  - td_h) as usize ;
    }
    println!("td_pages:{} mb",std::mem::size_of_val(&td_pages) / 1024);
    text_display.set_rich_text(td_pages[0].clone());

    let mut scrollbar = fltk::valuator::Scrollbar::new(377, 10, 15, 580, "");
    scrollbar.set_slider_size(0.05);
    scrollbar.set_maximum((count + td_h) as f64);
    // 添加滚动条拖动事件的回调函数
    let mut td = text_display.clone();
    let mut rich_11 = RichTextBuilder::new();
    
    for i in 0..window_size as usize {
        rich_11.append(
            &line_buffer[i + window_size as usize - window_pre_offset - td_h as usize],
            None,
        );
    }

    scrollbar.set_callback(move |s| {
        let pos = s.value();

        let mut page_index = PAGE_INDEX.load(std::sync::atomic::Ordering::Relaxed);
        let page_offset = ((pos  - (page_index * window_size) as f64) / window_size as f64).abs().ceil() as i32;

        if pos > ((page_index +1)*window_size - window_pre_offset as i32 - td_h - line_h).into() {
            println!(" pos:{pos} > ((offset:{page_index} +1)*window_size:{window_size} - window_pre_offset:{window_pre_offset} as i32 - td_h:{td_h} - line_h:{line_h}).into()");
            println!("more");
            let start = Instant::now();
            
            page_index += page_offset;
            if page_index >= td_pages.len() as i32 {
                page_index = (td_pages.len() - 1) as i32;
            }
            let rich_page = td_pages[page_index as usize].clone();
            PAGE_INDEX.store(page_index, std::sync::atomic::Ordering::Relaxed);
            td.set_rich_text(rich_page);
            //td.scroll(0, 0);
            let end = Instant::now();
            // 计算时间间隔
            let duration = end - start;
            // 输出执行时间（单位：纳秒）
            println!("Execution time: {} as_millis", duration.as_millis());
            
        } 
        else if pos < (page_index * window_size - window_pre_offset as i32 - td_h).into() {
            println!("pos:{pos} < (offset:{page_index} - window_pre_offset:{window_pre_offset} as i32 - td_h:{td_h})");
            println!("less");
            page_index -=page_offset;
            let rich_page = td_pages[page_index as usize].clone();
            PAGE_INDEX.store(page_index, std::sync::atomic::Ordering::Relaxed);
            td.set_rich_text(rich_page);
        }

        // 处理滚动条拖动事件，这里只是打印滚动条位置
        //println!("Scrollbar position: {}", pos);
        let text_display_offset = if page_index > 0 {
            pos as i32 - (page_index * window_size) + window_pre_offset as i32
        } else {
            pos as i32 - (page_index * window_size)
        };
        td.scroll(text_display_offset, 0);
        //SCROLL_BAR_POS.store(position as i32, std::sync::atomic::Ordering::Relaxed);

        println!("text_display_offset:{text_display_offset},Scrollbar scroll_pos: {}, offset * window_size:{}", pos,page_index * window_size);
    });

    text_display.handle(move |s, event| {
        if event == Event::MouseWheel {
            let dy = app::event_dy();
            let pos = scrollbar.value();
            let r = match dy {
                app::MouseWheel::Up => {
                    // let mut offset = PAGE_INDEX.load(std::sync::atomic::Ordering::Relaxed);
                    // if position > (offset + window_size - window_pre_offset as i32).into() {
                    //     rich_1.clear();
                    //     offset += window_size;
                    //     for i in 0..window_size as usize {
                    //         rich_1.append(
                    //             &line_buffer
                    //                 [i + offset as usize - window_pre_offset - td_h as usize],
                    //             None,
                    //         );
                    //     }
                    //     PAGE_INDEX.store(offset, std::sync::atomic::Ordering::Relaxed);
                    // }
                    // let pos = SCROLL_BAR_POS.load(std::sync::atomic::Ordering::Relaxed);
                    // SCROLL_BAR_POS.store(pos + 3, std::sync::atomic::Ordering::Relaxed);

                    // if pos + 3 > count - td_h {
                    //     SCROLL_BAR_POS.store(count - td_h, std::sync::atomic::Ordering::Relaxed);
                    // }
                    if pos + 3.0 > (count - td_h) as f64 {
                        scrollbar.set_value((count - td_h) as f64);
                    } else {
                        scrollbar.set_value(pos + 3.0);
                    }

                    println!("Scroll Position: {}", pos + 3.0);
                    println!("Mouse wheel up");
                    true
                }
                app::MouseWheel::Down => {
                    println!("Mouse wheel down");
                    // let pos = SCROLL_BAR_POS.load(std::sync::atomic::Ordering::Relaxed);
                    // SCROLL_BAR_POS.store(pos - 3, std::sync::atomic::Ordering::Relaxed);

                    // if pos - 3 < 0 {
                    //     SCROLL_BAR_POS.store(0, std::sync::atomic::Ordering::Relaxed);
                    // }
                    if pos - 3.0 < 0.0 {
                        scrollbar.set_value(0.0);
                    } else {
                        scrollbar.set_value(pos - 3.0);
                    }
                    println!("Scroll Position: {}", pos - 3.0);
                    true
                }
                _ => false,
            };
            //scrollbar.set_value(SCROLL_BAR_POS.load(std::sync::atomic::Ordering::Relaxed) as f64);
            return r; // 处理事件
        }
        false // 不处理其他事件
    });

    win.end();
    win.show();

    app.run().unwrap();
}
