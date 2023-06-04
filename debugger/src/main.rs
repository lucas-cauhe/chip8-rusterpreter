mod debugger;
mod components;
use debugger::{Debugger, Display};
use std::{env, io};



fn main () {

    /* let file: Vec<String> = env::args().collect();
    // initialize debugger
    let mut debugger = Debugger::new(file[0].as_str());
    let mut next_cmd = String::new();
    loop {
        // wait for a command
        // this call will block the current thread so if there are set timers, they won't work as expected
        if let Ok(0) = io::stdin().read_line(&mut next_cmd) {
            println!("Reached EOF");
            break;
        }

        debugger.execute(&next_cmd);
        // execute command
        
            // if command executes instruction and is timer instruction
                // display timer on one side

        // update chip and display
    } */
    let mut display = Display::setup();
    display.render_display(); 

}