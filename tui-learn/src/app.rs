pub struct App {
    pub title: String,
    pub cur_block_id: i32,
}

impl App {
    pub fn new(title: String) -> Self {
        App {
            title,
            cur_block_id: 0,
        }
    }
}
