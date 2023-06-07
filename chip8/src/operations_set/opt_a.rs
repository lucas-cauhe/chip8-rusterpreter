use super::operations_table::*;
use crate::Chip8;

/// Annn - LD I, addr
/// 
/// Set I = nnn.
/// 
/// The value of register I is set to nnn.
pub struct OptA {}
impl Executable for OptA {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        chip.set_i_register_value(specs.addr);
        chip.update_pc(None);
        Ok(())
    }
}