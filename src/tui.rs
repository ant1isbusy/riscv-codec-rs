use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self};

use crate::{decoder, encoder, format, util};

struct App {
    input: String,
    output: String,
}

pub fn run_tui() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App {
        input: String::new(),
        output: String::new(),
    };

    loop {
        terminal
            .draw(|f| {
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(size);

                // Input box
                let input_field = Paragraph::new(app.input.as_str()).block(
                    Block::default()
                        .title("Instruction Input")
                        .borders(Borders::ALL),
                );
                f.render_widget(input_field, chunks[0]);

                let lines: Vec<Line> = app
                    .output
                    .lines()
                    .map(|l| Line::from(Span::raw(l.to_string())))
                    .collect();

                let output_box = Paragraph::new(lines).block(
                    Block::default()
                        .title("Decoded Output")
                        .borders(Borders::ALL),
                );
                f.render_widget(output_box, chunks[1]);
            })
            .unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        let input_trimmed = app.input.trim();
                        if input_trimmed.eq_ignore_ascii_case("exit")
                            || input_trimmed.eq_ignore_ascii_case("q")
                            || input_trimmed.eq_ignore_ascii_case("quit")
                        {
                            break;
                        }

                        // Call your encoder/decoder logic
                        app.output = if util::is_hex(input_trimmed) {
                            match u32::from_str_radix(&input_trimmed[2..], 16) {
                                Ok(hex) => match decoder::decode(hex) {
                                    Ok(instr) => format::encode_instruction_as_string(&instr),
                                    Err(_) => "Error parsing hex input".to_string(),
                                },
                                Err(_) => "Error parsing hex input".to_string(),
                            }
                        } else {
                            match encoder::encode(input_trimmed) {
                                Ok(instr) => format::encode_instruction_as_string(&instr),
                                Err(e) => format!("Error encoding instruction: {:?}", e),
                            }
                        };

                        app.input.clear();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
