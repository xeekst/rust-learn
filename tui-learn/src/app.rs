use unicode_width::UnicodeWidthStr;

pub struct App {
    pub title: String,
    pub input_mode: InputMode,
    pub log_output_mode: LogsMode,
    pub input: String,
    pub log_rows: Vec<String>,
    pub scroll: u16,
    pub logs_ui_count: u16,

    pub api_status: NetworkStatus,
    pub task: Task,
}

pub struct Task {
    pub id: String,
    pub sub_task_id: String,
}

pub enum NetworkStatus {
    Connected,
    Disconnected,
}

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub enum LogsMode {
    Continue,
    Pause
}

impl App {
    pub fn new(title: String) -> Self {
        App {
            title,
            input_mode: InputMode::Normal,
            input: "".to_string(),
            log_rows: vec![],
            api_status: NetworkStatus::Connected,
            task: Task {
                id: "12312514213".to_string(),
                sub_task_id: "1231251321".to_string(),
            },
            scroll: 0,
            logs_ui_count: 0,
            log_output_mode: LogsMode::Continue,
        }
    }

    pub fn add_log(&mut self, log: &str) {
        self.log_rows.push(log.to_string().drain(..).collect());
    }
}
