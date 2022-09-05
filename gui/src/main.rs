use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use gui::{Button, Screen, Screen2};

fn main() {
    let screen = Screen {
        components: vec![Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }],
    };

    screen.run();
}
