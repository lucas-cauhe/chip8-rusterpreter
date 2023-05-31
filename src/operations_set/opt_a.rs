use super::operations_table::*;
use crate::Chip8;
pub struct OptA {}
impl Executable for OptA {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        chip.set_i_register_value(specs.addr);
        chip.update_pc(None);
        Ok(())
    }
}