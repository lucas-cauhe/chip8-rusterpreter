mod debugger;
mod components;
mod scaffold;
use debugger::{Debugger, Display};
use std::{env, io};



fn main () {

    /* let file: Vec<String> = env::args().collect();
    // initialize debugger
    let mut debugger = Debugger::new(file[0].as_str());
    let mut next_cmd = String::new();
    loop {
        // wait for a command
        next_cmd = debugger.receive_cmd().expect("Error receiving command");
        // cambiar por un try_recv()
        // this call will block the current thread so if there are set timers, they won't work as expected
        if let Ok(0) = io::stdin().read_line(&mut next_cmd) {
            println!("Reached EOF");
            break;
        }

        debugger.execute(&next_cmd);
        // execute command
        
            // if command executes instruction and is timer instruction
                // display timer on one side

        // update display
        debugger.update_screen();
    } */
    let chip = chip8::chip8::Chip8::new();
    let mut display = Display::new(&chip);
    display.render_display(); 

}

/*
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
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

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    let layout =
        Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    let mut is_valid = validate(&mut textarea);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            let widget = textarea.widget();
            f.render_widget(widget, chunks[0]);
        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: Key::Enter, ..
            } if is_valid => break,
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
                if textarea.input(input) {
                    is_valid = validate(&mut textarea);
                }
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
}*/