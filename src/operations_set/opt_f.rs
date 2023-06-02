use super::operations_table::*;
use crate::{Chip8, chip8::RoutinePurpose};
pub struct OptF {}
impl OptF {
    pub fn select_execution(&self, specs: OperationSpecs, chip: &mut Chip8) -> Option<()> {
        // constant is placed in the same bit-space as the nibble and ry in F operations
        match specs.constant {
            0x15 => Some(self.execute_x15(specs, chip)),
            0x18 => Some(self.execute_x18(specs, chip)),
            // ...
            _ => None// invalid nibble
        }
    }
    // LD DT VX
    fn execute_x15(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let rx = chip.get_register_value(specs.rx);
        let delay_timer_addr = chip.get_routine_addr(RoutinePurpose::DelayTimer);
        chip.set_delay_timer(rx, delay_timer_addr);
        chip.update_pc(None);
    }
    // LD ST VX
    fn execute_x18(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let rx = chip.get_register_value(specs.rx);
        let sound_timer_addr = chip.get_routine_addr(RoutinePurpose::SoundTimer);
        chip.set_sound_timer(rx, sound_timer_addr);
        chip.update_pc(None);
    }
}

impl Executable for OptF {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        match self.select_execution(specs, chip) {
            Some(()) => Ok(()),
            None => Err("OptF execution failed: ".to_string())
        }
    }
}
