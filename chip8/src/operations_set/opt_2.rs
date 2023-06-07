use super::operations_table::*;
use crate::Chip8;


/// 2nnn - CALL addr
///
/// Call subroutine at nnn.
///
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
pub struct Opt2 {}
impl Executable for Opt2 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        if Chip8::address_out_of_bounds(specs.addr) {
            Err("Address out of bounds in function call: Opt2.execute()".to_string())
        } else {
            chip.call_subroutine(specs.addr)?;
            Ok(())
        }
        
    }
}