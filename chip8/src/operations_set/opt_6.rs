use super::operations_table::*;
use crate::Chip8;

/// 6xkk - LD Vx, byte
/// Set Vx = kk.
/// 
/// The interpreter puts the value kk into register Vx.
pub struct Opt6 {}
impl Executable for Opt6 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        chip.set_register_value(specs.rx, specs.constant);
        chip.update_pc(None);
        Ok(())    
    }
}