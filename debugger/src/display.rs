use chip8::{self, chip8::Chip8};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use std::io::{self, Stdout};

use crate::components::{
    registers::RegistersComponent, 
    screen::ScreenComponent, 
    text::TextComponent, 
    command::CommandComponent, 
    timers::{DelayTimerComponent, SoundTimerComponent}
};
use crate::scaffold::Scaffold;
type DefaultTerminal = Terminal<CrosstermBackend<Stdout>>;


pub struct Display {
    pub term: DefaultTerminal,
    pub distribution: Option<Scaffold>, // gets constructed the first time the display is activated
    pub chip_status: RegistersComponent,
    pub screen: ScreenComponent,
    pub text: TextComponent,
    pub command: CommandComponent,
    pub delay_timer: Option<DelayTimerComponent>,
    pub sound_timer: Option<SoundTimerComponent>
}
impl Display {

    pub fn new(chip: &Chip8) -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        
        terminal.clear().unwrap();
        Self { 
            term: terminal, 
            distribution: None, 
            chip_status: RegistersComponent::new(
                (0..16).into_iter().map(|ind| chip.get_register_value(ind) ).collect::<Vec<u8>>().as_slice()
            ), 
            screen: ScreenComponent::new(),
            text: TextComponent::new("tests/mock_program.txt"),
            command: CommandComponent::new(),
            delay_timer: None,
            sound_timer: None
        }
    }
    pub fn render_display(&mut self) {
        self.term.draw(|rect| {
            
            let size = rect.size();
            if let None = self.distribution {
                self.distribution = Some(Scaffold::new(size));
            }
            
            let dist = self.distribution.as_ref().unwrap();
            rect.render_widget(self.chip_status.style.clone(), dist.registers);
            rect.render_widget(self.text.style.clone(), dist.code);
            rect.render_widget(self.command.style.widget(), dist.command);
            rect.render_widget(self.screen.style.clone(), dist.output);
            if let Some(timer) = self.sound_timer.as_ref() {
                rect.render_widget(timer.style.clone(), dist.sound_timer);
            }
            if let Some(timer) = self.delay_timer.as_ref() {
                rect.render_widget(timer.style.clone(), dist.delay_timer);
            }
        }).unwrap();
    }

    pub fn show_error(&mut self, msg: &str) {
        self.term.draw(|rect| {
            // display error lower widget with error message
        }).unwrap();
    }
}