use chip8::chip8::Chip8;
use tui::{ 
    widgets::{Borders, BorderType, Block, List, ListItem}, 
    style::{Color, Style, Modifier}, text::{Spans, Span}
};


pub struct RegistersComponent {
    pub style: List<'static>
}

impl RegistersComponent {
    pub fn new (reg_values: &[u8]) -> Self {
        let items: Vec<_> = reg_values.iter().enumerate().map(|(ind, val)| ListItem::new(Spans::from(vec![Span::styled(
            ind.to_string() + ": " + &format!("{:#06x}", val),
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
        Self { 
            style: registers
        }
    }
}