use super::operations_table::*;
use crate::Chip8;

/// 3xkk - SE Vx, byte
///
///Skip next instruction if Vx = kk.
///
///The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
pub struct Opt3 {}
impl Executable for Opt3 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        if chip.get_register_value(specs.rx) == specs.constant {
            chip.update_pc(None);
        }
        chip.update_pc(None);
        Ok(())    
    }
}