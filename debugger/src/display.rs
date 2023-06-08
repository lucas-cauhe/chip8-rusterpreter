use tui::{
    backend::CrosstermBackend,
    Terminal, widgets::{Paragraph, List, Block, Borders, BorderType, ListItem}, style::{Color, Style, Modifier}, text::{Spans, Span}
};
use std::{io::{self, Stdout}, sync::{Mutex, Arc}};

use crate::components::{
    registers::RegistersComponent, 
    screen::ScreenComponent, 
    text::TextComponent, 
    command::CommandComponent, 
    timers::{DelayTimerComponent, SoundTimerComponent}
};
use crate::scaffold::Scaffold;
pub type DefaultTerminal = Terminal<CrosstermBackend<Stdout>>;


pub struct Display {
    pub term: Arc<Mutex<DefaultTerminal>>,
    pub distribution: Option<Scaffold>, // gets constructed the first time the display is activated
    pub chip_status: RegistersComponent,
    pub screen: ScreenComponent,
    pub text: TextComponent,
    pub command: CommandComponent,
    pub delay_timer: Option<DelayTimerComponent>,
    pub sound_timer: Option<SoundTimerComponent>
}
impl Display {

    pub fn new(file: &str) -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        
        terminal.clear().unwrap();
        Self { 
            term: Arc::new(Mutex::new(terminal)), 
            distribution: None, 
            chip_status: RegistersComponent::new(), 
            screen: ScreenComponent::new(),
            text: TextComponent::new(file),
            command: CommandComponent::new(),
            delay_timer: None,
            sound_timer: None
        }
    }
    pub fn render_display(&mut self, current_line: usize) {
        let mut term_lck = self.term.lock().unwrap();
        term_lck.draw(|rect| {
            
            let size = rect.size();
            if let None = self.distribution {
                self.distribution = Some(Scaffold::new(size));
            }
            let mut arrows: Vec<_> = (0..(self.text.text.lines().collect::<Vec<&str>>().len())).into_iter().map(|_| ListItem::new(Spans::from(vec![Span::styled(
                "",
                Style::default(),
            )]))).collect();
            
            arrows[current_line] = ListItem::new(Spans::from(vec![Span::styled(
                "->",
                Style::default(),
            )]));
            let arrow_list = List::new(arrows)
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
                    .border_type(BorderType::Plain),
            );
            let dist = self.distribution.as_ref().unwrap();
            rect.render_widget(self.screen.style.clone(), dist.output);
            rect.render_widget(self.chip_status.style.clone(), dist.registers);
            rect.render_widget(self.text.style.clone(), dist.code);
            rect.render_widget(self.command.style.widget(), dist.command);
            rect.render_widget(arrow_list, dist.arrows);
            if let Some(timer) = self.sound_timer.as_ref() {
                rect.render_widget(timer.style.lock().unwrap().widget(), dist.sound_timer);
            }
            if let Some(timer) = self.delay_timer.as_ref() {
                rect.render_widget(timer.style.clone(), dist.delay_timer);
            }
        }).unwrap();
    }

    pub fn show_error(&mut self, msg: &str) {
        self.text.style = Paragraph::from(self.text.style.clone())
            .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red))
            .title("Code Error: ".to_string() + msg)
            .border_type(BorderType::Plain),);
    }
}