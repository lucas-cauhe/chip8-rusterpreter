use super::operations_table::*;
use crate::Chip8;

/// 9xy0 - SNE Vx, Vy
///
///Skip next instruction if Vx != Vy.
///
///The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
pub struct Opt9 {}
impl Executable for Opt9 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        if chip.get_register_value(specs.rx) != chip.get_register_value(specs.ry) {
            chip.update_pc(None);
        }
        chip.update_pc(None);
        Ok(())    
    }
}