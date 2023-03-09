
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



#[derive(Debug, Clone)]
pub struct UserInterface {
    pub terminal: SimpleTerminal,
}


impl UserInterface {
    pub fn make_window() -> Self {
	let mut fl2rust_widget_0 = Window::new(683, 249, 499, 395, None);
	fl2rust_widget_0.end();
	fl2rust_widget_0.show();
	let mut terminal = SimpleTerminal::new(45, 24, 360, 257, None);
	terminal.insert("\x1b[93mError\x1b[0m");
	terminal.insert("\x1b[93mError\x1b[0m");
	terminal.insert("\x1b[93mError\x1b[0m");
	terminal.insert("\x1b[93mError\x1b[0m");
	terminal.insert("\x1b[93mError\x1b[0m");
	terminal.insert("\x1b[93mError\x1b[0m");
	fl2rust_widget_0.add(&terminal);
	Self { terminal, }
    }
}


