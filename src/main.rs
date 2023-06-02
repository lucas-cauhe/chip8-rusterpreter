mod chip8;
mod timers;
mod operations_set;

use chip8::{Chip8, ProgramType};
use core::str;
use std::env;
use std::io;

/* fn update_screen(gfx: &[Vec<u8>]) {
    gfx.iter().map(|row| row.iter().map(|px8| {
        let mut out = io::stdout().write_all();
        let mut out_lck = out.lock();
        out_lck.writ
    }))
} */


fn main() {
    
    let mut chip = Chip8::new();
    let file: Vec<String> = env::args().collect();

    chip.load_program(ProgramType::Main(file[0].as_str())).expect("Error loading program: ");

    loop {
        if let Err(eop) = chip.execute_cycle() {
            println!("Program terminated with status: {:?}", eop.status);
            break;
        }

        // check if you need to display gfx on screen
        let vf = chip.get_register_value(15);
        if vf & 0x80 == 0x80 {
            // update screen
            //update_screen(chip.get_gfx());

            // put the draw flag down
            chip.set_register_value(15, vf & 0x7F);
        }
    }
    

}
