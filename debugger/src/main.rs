mod debugger;
mod components;
mod scaffold;
mod display;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use debugger::Debugger;
use std::env;



fn main () {

    let file: Vec<String> = env::args().collect();
    enable_raw_mode().unwrap();
    // crossterm::execute!(io::stdout().lock(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    // initialize debugger
    let mut debugger = Debugger::new(file[1].as_str());
    let mut next_cmd;
    debugger.update_screen();
    
    loop {
        // wait for a command
       
        next_cmd = debugger.receive_cmd().expect("Error receiving command");
        if next_cmd == "exit".to_string() {
            break;
        }
        
        if next_cmd != "".to_string() {
            debugger.execute(&next_cmd).expect("Error executing: ");
        }
        // update display
        debugger.update_screen();
        
    }
    disable_raw_mode().unwrap();
}