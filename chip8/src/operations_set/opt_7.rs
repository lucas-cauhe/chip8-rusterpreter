use super::operations_table::*;
use crate::Chip8;

/// 7xkk - ADD Vx, byte
/// Set Vx = Vx + kk.
/// 
/// Adds the value kk to the value of register Vx, then stores the result in Vx.
pub struct Opt7 {}
impl Executable for Opt7 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        let rx_value = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, specs.constant + rx_value);
        chip.update_pc(None);
        Ok(())    
    }
}