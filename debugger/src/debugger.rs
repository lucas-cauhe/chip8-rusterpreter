use chip8::{self, chip8::{Chip8, ProgramType}, timers::Signals};
use crossterm::{
    self,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture}
};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use std::io::{self, Stdout};
use tui_textarea::{Input, Key};

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

pub struct Debugger {
    chip: Chip8,
    display: Display,
    next_breakpoint: Option<u32>,
    current_line: u32
}

impl Debugger {
    pub fn new(program: &str) -> Self {
        let mut chip = Chip8::new();
        chip.load_program(ProgramType::Main(program)).unwrap();
        Self { 
            display: Display::new(&chip),
            chip,
            next_breakpoint: None,
            current_line: 0
        }
    }

    pub fn receive_cmd(&self) -> Result<String, String> {
        let cmd = self.display.command.rx.recv().expect("Error receiving command: ");
        Ok(cmd)
    }

    pub fn execute(&mut self, cmd: &String) -> Result<(), String> {
        match cmd.as_str() {
            "n" => {
                // copy the working version of the loop action in chip8 main.rs
                // tweak the screen variables
                // if instruction set up a timer, create it in the display -> self.display.sound_timer = Some(SoundTimerComponent::new()),
                // update current_line
                todo!()
            },
            "r" => {
                // change 100 default values for the number of lines in a program in both cases
                while self.current_line != self.next_breakpoint.unwrap_or(100) && self.current_line != /* EOF */ 100 {
                    if let Err(what) = self.execute(&"n".to_string()) {
                        self.display.show_error(what.as_str());
                        break;
                    } 
                }
                Ok(())
            },
            other_cmd => {
                let cmd_parts: Vec<&str> = other_cmd.split(' ').collect();
                match cmd_parts[0] {
                    "b" => {
                        match cmd_parts[1] {
                            "-l" => {
                                self.next_breakpoint = Some(cmd_parts[2].parse().unwrap());
                                Ok(())
                            },
                            "-p" => {
                                if let Some(line) = self.display.text.find_definition(cmd_parts[2]){
                                    self.next_breakpoint = Some(line);
                                    Ok(())
                                } else {
                                    Err("name of definition not found".to_string())
                                }
                            },
                            _ => Err("flag not found".to_string())
                        }
                    },
                    "stop" => {
                        self.chip.send_signal(Signals::STP, cmd_parts[1])?;
                        Ok(())
                    },
                    "resume" => {
                        self.chip.send_signal(Signals::RES, cmd_parts[1])?;
                        Ok(())
                    },
                    _ => Err("Command not found".to_string())
                }
            }
            
        }
    }

    pub fn update_screen(&mut self) {
        self.display.render_display();
    }

}

