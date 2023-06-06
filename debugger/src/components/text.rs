use tui::{
    layout::Alignment, 
    widgets::{Paragraph, Borders, BorderType, Block}, 
    style::{Color, Style}
};
use std::{fs, borrow::BorrowMut};
pub struct TextComponent {
    pub style: Paragraph<'static>,
    pub text: String
}

impl TextComponent {
    pub fn new(file: &str) -> Self {
        let text = fs::read_to_string(file).unwrap();
        let code = Paragraph::new(text.clone())
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Code")
                    .border_type(BorderType::Plain),
            );
        Self {
            style: code,
            text
        }
    }

    pub fn find_definition(&self, def: &str) -> Option<u32> {
        // search for label with name = def
        let matches: Vec<(usize, &str)> = self.text.lines().enumerate().filter(|(_, line)| line.find(def).is_some() ).collect();
        if matches.len() > 0 {
            if matches.len() > 1 {
                println!("Label specified is ambiguous, selected first match");
            }
            Some(matches[0].0 as u32)
        } else {
            None
        }
    }
}