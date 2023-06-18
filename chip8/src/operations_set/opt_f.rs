use super::operations_table::*;
use crate::{Chip8, chip8::RoutinePurpose};
pub struct OptF {}
impl OptF {
    pub fn select_execution(&self, specs: OperationSpecs, chip: &mut Chip8) -> Option<()> {
        // constant is placed in the same bit-space as the nibble and ry in F operations
        match specs.constant {
            0x15 => Some(self.execute_x15(specs, chip)),
            0x18 => Some(self.execute_x18(specs, chip)),
            0x55 => Some(self.execute_x55(specs, chip)),
            // ...
            _ => None// invalid nibble
        }
    }
    
    /// Fx15 - LD DT, Vx
    ///
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    fn execute_x15(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let rx = chip.get_register_value(specs.rx);
        let delay_timer_addr = chip.get_routine_addr(RoutinePurpose::DelayTimer);
        chip.set_delay_timer(rx, delay_timer_addr);
        chip.update_pc(None);
    }

    /// Fx18 - LD ST, Vx
    ///
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    fn execute_x18(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let rx = chip.get_register_value(specs.rx);
        let sound_timer_addr = chip.get_routine_addr(RoutinePurpose::SoundTimer);
        chip.set_sound_timer(rx, sound_timer_addr);
        chip.update_pc(None);
    }

    /// Fx55 - LD [I], Vx
    /// 
    ///Store registers V0 through Vx in memory starting at location I.
    ///
    ///The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn execute_x55(&self, specs: OperationSpecs, chip: &mut Chip8) {
        (0..specs.rx+1).into_iter().for_each(|r| chip.set_memory_value(chip.get_register_value(r)));
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
