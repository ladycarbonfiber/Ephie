use std::str;
/// This example is taken from https://raw.githubusercontent.com/fdehau/tui-rs/master/examples/user_input.rs
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use transport_layer::command::{Command, WRITE_DELIM};

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    /// Active User id
    user: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            user: "1".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app).await;

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

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::<B>(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let mut stream = TcpStream::connect("127.0.0.1:8888").await.unwrap();
                        app.messages.push(app.input.value().into());
                        //let mut parts_iter = app.input.value().clone();

                        let mut parts = app
                            .input
                            .value()
                            .split_whitespace()
                            .take(2)
                            .collect::<Vec<&str>>();
                        // allows us to have white space in what we write
                        let maybe_write_part = app
                            .input
                            .value()
                            .split_whitespace()
                            .skip(2)
                            .collect::<Vec<&str>>()
                            .join(" ");
                        if !maybe_write_part.is_empty() {
                            parts.push(&maybe_write_part.as_str());
                        }

                        let command = match parts.len() {
                            0 => Command::UNKNOWN,
                            1 => Command::from(app.input.value()),
                            3 => {
                                if parts[0] == "write" || parts[0] == "cp" || parts[0] =="mv"{
                                    let mut combined = String::new();
                                    combined.push_str(parts[1]);
                                    combined.push_str(WRITE_DELIM);
                                    combined.push_str(parts[2]);
                                    Command::from((parts[0], combined.as_str()))
                                } else {
                                    Command::UNKNOWN
                                }
                            }
                            _ => Command::from((parts[0], parts[1])),
                        };
                        //For debugging
                        //app.messages.push(format!("{:?}", &command.to_bytes()));
                        match command {
                            Command::UNKNOWN => {
                                app.messages.push(format!("command unknown"));
                            }
                            Command::SU(user) =>{
                                app.user = user;
                            }
                            _ => {
                                let user_code = match app.user.as_str() {
                                    "1" => 1u8,
                                    "2" => 2u8,
                                    "3" => 3u8,
                                    _ =>u8::MAX,
                                };
                                stream.write_all(&command.to_bytes(user_code)).await.unwrap();
                                let mut buff = [0; 1];
                                let read_payload = stream
                                    .read_exact(&mut buff)
                                    .await
                                    .expect("Failed to read data from socket");
                                if read_payload > 0 {
                                    let payload_len = buff[0];
                                    let mut payload_buffer = vec![0u8; payload_len as usize];
                                    let read_data_len = stream
                                        .read_exact(&mut payload_buffer)
                                        .await
                                        .expect("Failed to read data from socket");
                                    if read_data_len > 0 {
                                        let s = match str::from_utf8(
                                            &payload_buffer[0..payload_len as usize],
                                        ) {
                                            Ok(v) => v,
                                            Err(_) => "error",
                                        };
                                        app.messages.push(format!("Recieved: {}", s));
                                    }
                                }
                            }
                        }

                        app.input.reset();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Line::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[2]);
}
