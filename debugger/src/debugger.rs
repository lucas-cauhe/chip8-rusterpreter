use std::sync::{Arc, mpsc::TryRecvError};

use chip8::{self, chip8::{Chip8, ProgramType}, timers::Signals};
use crate::display::Display;
use crate::components::
    timers::{DelayTimerComponent, SoundTimerComponent};
pub struct Debugger {
    chip: Chip8,
    display: Display,
    next_breakpoint: Option<u32>,
    current_line: u32
}

impl Debugger {
    pub fn new(program: &str) -> Self {
        let mut chip = Chip8::new();
        chip.load_program(ProgramType::Main(program), None, None).unwrap();
        Self { 
            display: Display::new(program),
            chip,
            next_breakpoint: None,
            current_line: 0
        }
    }

    pub fn receive_cmd(&self) -> Result<String, String> {
        match self.display.command.rx.try_recv() {
            Ok(cmd) => Ok(cmd),
            Err(TryRecvError::Empty) => Ok("".to_string()),
            _ => Err("disconnected".to_string())
        }
    }

    fn next_instruction_sets_timer(&self) -> Option<&str> {
        if let Some(_) = self.display.text.text.lines().collect::<Vec<_>>()[self.current_line as usize].find("ST") {
            Some("sound")
        }
        else if let Some(_) = self.display.text.text.lines().collect::<Vec<_>>()[self.current_line as usize].find("DT") {
            Some("delay")
        }
        else {
            None
        }
    }

    fn get_next_instruction_timer_value(&self) -> u32 {
        let rx = self.display.text.text.lines().collect::<Vec<_>>()[self.current_line as usize].split(' ').nth(2).unwrap();
        self.chip.get_register_value(rx.chars().nth(1).unwrap().to_digit(10).unwrap() as u8 ) as u32
    }

    pub fn execute(&mut self, cmd: &String) -> Result<(), String> {
        
        
        match cmd.as_str() {
            "n" => {
                // copy the working version of the loop action in chip8 main.rs
                // tweak the screen variables
                
                if let Some(timer) = self.next_instruction_sets_timer() { 
                    // get counter value
                    let counter = self.get_next_instruction_timer_value();
                    match timer {
                        "delay" => self.display.delay_timer = Some(DelayTimerComponent::new(counter)),
                        "sound" => self.display.sound_timer = Some(SoundTimerComponent::new(counter)),
                        _ => { }
                        
                    }
                }
                if let Err(eop) = self.chip.execute_cycle() {
                    return Err("Program terminated with status: ".to_string() + &eop.status.to_string())
                }
                
                // check if you need to display gfx on screen
                let vf = self.chip.get_register_value(15);
                if vf & 0x80 == 0x80 {
                    // update screen
                    //update_screen(chip.get_gfx());
        
                    // put the draw flag down
                    self.chip.set_register_value(15, vf & 0x7F);
                }
                // update current_line
                //  this is incorrect due to jumps
                self.current_line += 1;

                Ok(())
            },
            "r" => {
                let last_line = self.display.text.text.lines().collect::<Vec<&str>>().len() as u32;
                while self.current_line < self.next_breakpoint.unwrap_or(last_line+1) 
                && self.current_line < last_line+1 {
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
                        // if you try to use the original timers, since their frequency is much higher, count will reach 0 before you type the command
                        // hence errors will arise
                        //self.chip.send_signal(Signals::STP, cmd_parts[1])?;
                        if cmd_parts[1] == "sound" {
                            self.display.sound_timer.as_ref().unwrap().tx.send(Signals::STP).expect("Error sending signal to simulation thread");
                        } else {
                            self.display.delay_timer.as_ref().unwrap().tx.send(Signals::STP).expect("Error sending signal to simulation thread");
                        }
                        
                        Ok(())
                    },
                    "resume" => {
                        //self.chip.send_signal(Signals::RES, cmd_parts[1])?;
                        if cmd_parts[1] == "sound" {
                            self.display.sound_timer.as_ref().unwrap().tx.send(Signals::RES).expect("Error sending signal to simulation thread");
                        } else {
                            self.display.delay_timer.as_ref().unwrap().tx.send(Signals::RES).expect("Error sending signal to simulation thread");
                        }
                        Ok(())
                    },
                    _ => Err("Command not found".to_string())
                }
            }
            
        }
    }

    pub fn update_screen(&mut self) {

        self.display.chip_status.update_component(
            (0..16).into_iter().map(|ind| self.chip.get_register_value(ind) ).collect::<Vec<u8>>().as_slice()
        );

        match self.display.sound_timer.take() {
            Some(timer) if *timer.time_left.lock().unwrap() != 0_u32 => self.display.sound_timer = Some(timer),
            _ => { }
        }
        self.display.render_display(self.current_line as usize);
    }
}

