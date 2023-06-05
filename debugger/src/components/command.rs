use std::{sync::mpsc::{Receiver, self}, thread};

use tui::{
    layout::{Alignment}, 
    widgets::{Borders, BorderType, Block}, 
    style::{Color, Style}
};
use tui_textarea::{TextArea, Input, Key};


pub struct CommandComponent {
    //pub style: Paragraph<'static>
    pub style: TextArea<'static>,
    pub command: String,
    pub rx: Receiver<String>
}

impl CommandComponent {
    pub fn new() -> Self {
        let mut command = TextArea::default();
        command.set_cursor_line_style(Style::default());
        //command.set_style(Style::default().fg(Color::LightCyan));
        command.set_alignment(Alignment::Left);
        command.set_block(Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Command")
                    .border_type(BorderType::Plain));
        let (sx, rx) = mpsc::channel::<String>();
        // thread for block reading stdin
        thread::spawn(move || {
            let mut cmd = "".to_string();
            loop {
                match crossterm::event::read().unwrap().into()  {
                    // hit Ctrl+C to exit
                    Input { key: Key::Char('c'), ctrl: true, .. } => break,
                    Input { key: Key::Char(ch), .. } => {
                        cmd.push(ch);
                    },
                    Input { key: Key::Enter, .. } => {
                        sx.send(cmd).expect("Error sending command");
                        cmd = "".to_string();
                    }, 
                    _ => { }
                };
            }
        });
        Self { 
            style: command,
            command: String::from(""),
            rx
        }
    }
}