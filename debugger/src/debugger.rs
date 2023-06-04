use chip8::{self, chip8::{Chip8, ProgramType}};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, }
};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
    layout::{Layout, Direction, Constraint, Alignment}, 
    widgets::{Paragraph, Borders, BorderType, Block, List, ListItem}, 
    style::{Color, Style, Modifier}, text::{Spans, Span}
};
use std::{io::{self, Stdout}, fs};
use tui_textarea::TextArea;

use crate::components::registers_component::RegistersComponent;

pub struct Display {
    term: Terminal<CrosstermBackend<Stdout>>,
    chip_status: RegistersComponent
    // text
    // timer_miniscreen: Option<>
}
impl Display {

    pub fn setup() -> Self {
        // enable_raw_mode().unwrap();
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        Self { term: terminal, chip_status: RegistersComponent::new() }
    }
    pub fn render_display(&mut self) {
        // hardcoded style for now
        disable_raw_mode().unwrap();
        loop {
            self.term.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(10),
                            Constraint::Min(2),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(size);
                let command = Paragraph::new("Command")
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Command")
                            .border_type(BorderType::Plain),
                    );
                let chip = chip8::chip8::Chip8::new();
                let items: Vec<_> = (0..16).into_iter().map(|ind| ListItem::new(Spans::from(vec![Span::styled(
                    ind.to_string() + ": " + &format!("{:#06x}", chip.get_register_value(ind)),
                    Style::default(),
                )]))).collect();
                let registers = List::new(items)
                    .style(Style::default().fg(Color::LightCyan))
                    .highlight_style(
                        Style::default()
                            .bg(Color::Yellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Regsiters")
                            .border_type(BorderType::Plain),
                    );
                let text = fs::read_to_string("tests/mock_program.txt").unwrap();
                let code = Paragraph::new(text)
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Code")
                            .border_type(BorderType::Plain),
                    );
                let screen = Paragraph::new("Output")
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Output")
                            .border_type(BorderType::Plain),
                    );
                let middle = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Percentage(25),
                            Constraint::Min(10)
                        ].as_ref(),
                    )
                    .split(chunks[1]);
                let sound_timer = Paragraph::new("Sound Timer")
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Sound Timer")
                            .border_type(BorderType::Plain),
                    );
                let delay_timer = Paragraph::new("Delay Timer")
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Delay Timer")
                            .border_type(BorderType::Plain),
                    );
                let timer_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50)
                        ].as_ref(),
                    )
                    .split(middle[1]);
                let timer_layout2 = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50)
                        ].as_ref(),
                    )
                    .split(timer_layout[1]);
                let timer_layout3 = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50)
                        ].as_ref(),
                    )
                    .split(timer_layout2[1]);
                rect.render_widget(registers, middle[0]);
                rect.render_widget(code, middle[1]);
                rect.render_widget(command, chunks[2]);
                rect.render_widget(screen, chunks[0]);
                rect.render_widget(sound_timer, timer_layout3[0]);
                rect.render_widget(delay_timer, timer_layout3[1]);
            }).unwrap();
        }
    }
}

pub struct Debugger {
    chip: Chip8,
    code_line: u32,
    display: Display
}

impl Debugger {
    pub fn new(program: &str) -> Self {
        let mut chip = Chip8::new();
        chip.load_program(ProgramType::Main(program)).unwrap();
        Self { 
            chip,
            code_line: 0, 
            display: Display::setup()
        }
    }

    pub fn execute(&mut self, cmd: &String) {
        match cmd.as_str() {
            "n" => print!("ok"),
            _ => print!("ak")
        }
    }

}

