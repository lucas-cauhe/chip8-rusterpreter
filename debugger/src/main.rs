mod debugger;
mod components;
mod scaffold;
mod display;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture}};
use debugger::Debugger;
use std::{env, io};



fn main () {

    let file: Vec<String> = env::args().collect();
    enable_raw_mode().unwrap();
    // crossterm::execute!(io::stdout().lock(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    // initialize debugger
    let mut debugger = Debugger::new(file[1].as_str());
    let mut next_cmd;
    debugger.update_screen();
    
    loop {
        // wait for a command
        // cambiar por un try_recv()
        next_cmd = debugger.receive_cmd().expect("Error receiving command");
        if next_cmd == "exit".to_string() {
            break;
        }
        debugger.execute(&next_cmd).expect("Error executing: ");
        // update display
        debugger.update_screen();
    }
    disable_raw_mode().unwrap();
    // crossterm::execute!(
    //     debugger.display.term.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // ).unwrap();
    // debugger.display.term.show_cursor().unwrap();
}

/* 
mod components;
mod debugger;
mod scaffold;
mod display;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::{io, thread};
use std::sync::mpsc;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};

fn validate(textarea: &mut TextArea) -> bool {
    if let Err(err) = textarea.lines()[0].parse::<f64>() {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("ERROR: {}", err)),
        );
        false
    } else {
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(Block::default().borders(Borders::ALL).title("OK"));
        true
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let (tx, rx) = mpsc::channel::<(String, Option<Input>)>();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    let layout =
        Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    let mut is_valid = validate(&mut textarea);

    thread::spawn(move || {
        loop {
            match crossterm::event::read().unwrap().into() {
                Input { key: Key::Esc, .. } => {
                    tx.send(("exit".to_string(), None));
                    break;
                },
                Input {
                    key: Key::Enter, ..
                } if is_valid => {
                    tx.send(("exit".to_string(), None));
                    break;
                },
                Input {
                    key: Key::Char('m'),
                    ctrl: true,
                    ..
                }
                | Input {
                    key: Key::Enter, ..
                } => {}
                input => {
                    // TextArea::input returns if the input modified its text
                    /* if textarea.input(input) {
                        is_valid = validate(&mut textarea);
                    } */
                    tx.send(("validate".to_string(), Some(input)));
                }
            }
        }
    });

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            let widget = textarea.widget();
            f.render_widget(widget, chunks[0]);
        })?;
        if let Ok(act) = rx.try_recv() {
            match act {
                (exit, None) if exit == "exit" => break,
                (validated, Some(input)) if validated == "validate" => {
                    if textarea.input(input) {
                        is_valid = validate(&mut textarea);
                    }
                },
                _ => { }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Input: {:?}", textarea.lines()[0]);
    Ok(())
} */