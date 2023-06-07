use super::operations_table::*;
use crate::Chip8;

/// 1nnn - JP addr
/// 
/// Jump to location nnn.
///
/// The interpreter sets the program counter to nnn.
 
pub struct Opt1 {}
impl Executable for Opt1 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        if Chip8::address_out_of_bounds(specs.addr) {
            Err("Address out of bounds in function call: Opt1.execute() ".to_string())
        }
        else {
            chip.update_pc(Some(specs.addr));
            Ok(())
        }
        
    }
}