use crate::app::{self, App, InputMode, LogsMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Quick Start Tui".to_string());
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        match event::read()? {
            Event::Key(key) => match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                        app.log_output_mode = LogsMode::Pause;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::F(10) => {
                        app.log_output_mode = LogsMode::Continue;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.add_log(app.input.clone().as_str());
                        app.add_log("[INFO][2023-03-19][12:54:54][fern_log_learn] Hello,Heafaf22llo,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello,Hello, \r\nworld!");
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            },
            Event::Mouse(mouse) => match mouse.kind {
                event::MouseEventKind::ScrollDown => {
                    app.scroll += 1;
                    app.log_output_mode = LogsMode::Pause;
                    // if app.scroll > app.log_rows.len() as u16 {
                    //     app.scroll = if app.log_rows.len() - 1 > 0 {
                    //         (app.log_rows.len() - 1) as u16
                    //     } else {
                    //         0
                    //     };
                    // }
                }
                event::MouseEventKind::ScrollUp => {
                    app.log_output_mode = LogsMode::Pause;
                    if app.scroll > 0 {
                        app.scroll -= 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());

    render_input(f, app, chunks[0]);
    render_logs(f, app, chunks[1]);
    render_status(f, app, chunks[2]);
}

fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input: <E:edit> | <Esc: exit edit> "),
        );

    f.render_widget(input, area);

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
    }
}

fn render_logs<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default().title(" Logs ").borders(Borders::ALL);

    let window_w = f.size().width;
    let mut r_count = 0;
    let mut text: Vec<Spans> = vec![];
    let mut index = 1;

    for line in app.log_rows.iter() {
        let mut space = " ".to_string();
        for _ in 0..(app.log_rows.len().to_string().width() - index.to_string().width()) {
            space += " ";
        }

        r_count += line.width() as u16 / window_w + 1;
        text.push(Spans::from(vec![
            Span::styled(format!("{index}{space}"), Style::default().fg(Color::Green)),
            Span::raw(line.as_str()),
        ]));
        index += 1;
    }
    app.logs_ui_count = r_count;
    // let text: Vec<Spans> = app
    //     .log_rows
    //     .iter()
    //     .map(|l| Spans::from(l.as_str()))
    //     .collect();
    let paragraph = match app.log_output_mode {
        LogsMode::Continue => Paragraph::new(text)
            .block(block)
            .scroll((r_count, 0))
            .wrap(Wrap { trim: true }),
        LogsMode::Pause => Paragraph::new(text)
            .block(block)
            .scroll((app.scroll, 0))
            .wrap(Wrap { trim: true }),
    };

    f.render_widget(paragraph, area);
}

fn render_status<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default().title(" Status ").borders(Borders::ALL);

    let status_text = Spans::from(vec![
        Span::styled(" api: ", Style::default().add_modifier(Modifier::BOLD)),
        match app.api_status {
            app::NetworkStatus::Connected => {
                Span::styled("Connected", Style::default().fg(Color::LightGreen))
            }
            app::NetworkStatus::Disconnected => {
                Span::styled("Disconnected", Style::default().fg(Color::LightRed))
            }
        },
        Span::styled("   task: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(app.task.id.clone(), Style::default()),
        Span::styled(
            "   subtask: ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            app.task.sub_task_id.clone(),
            Style::default().fg(Color::Gray),
        ),
        Span::styled("   count: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(
            app.log_rows.len().to_string().clone(),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(
            "   logs_ui_count: ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            app.logs_ui_count.to_string(),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(
            "   logs_output_status: ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{:?}", app.log_output_mode),
            match app.log_output_mode {
                LogsMode::Continue => Style::default().fg(Color::LightGreen),
                LogsMode::Pause => Style::default().fg(Color::Yellow),
            },
        ),
    ]);
    let paragraph = Paragraph::new(status_text)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}
