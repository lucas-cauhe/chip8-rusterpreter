use super::operations_table::*;
use crate::Chip8;

/// 5xy0 - SE Vx, Vy
/// Skip next instruction if Vx = Vy.
/// 
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.

pub struct Opt5 {}
impl Executable for Opt5 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        if chip.get_register_value(specs.rx) == chip.get_register_value(specs.ry) {
            chip.update_pc(None);
        }
        chip.update_pc(None);
        Ok(())    
    }
}