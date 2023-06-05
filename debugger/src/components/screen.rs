use tui::{
    layout::Alignment, 
    widgets::{Paragraph, Borders, BorderType, Block}, 
    style::{Color, Style}
};


pub struct ScreenComponent {
    pub style: Paragraph<'static>
}

impl ScreenComponent {
    pub fn new() -> Self {
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
        Self { 
            style: screen
        }
    }
}

