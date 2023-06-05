use tui::{
    layout::Alignment, 
    widgets::{Paragraph, Borders, BorderType, Block}, 
    style::{Color, Style}
};
pub struct DelayTimerComponent {
    pub style: Paragraph<'static>
}
impl DelayTimerComponent {
    pub fn new() -> Self {
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
        Self { 
            style: delay_timer
        }
    }
}

pub struct SoundTimerComponent {
    pub style: Paragraph<'static>
}
impl SoundTimerComponent {
    pub fn new() -> Self {
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
        Self { 
            style: sound_timer
        }
    }
}