use super::operations_table::*;
use crate::Chip8;

/// Bnnn - JP V0, addr
/// 
/// Jump to location nnn + V0.
/// 
/// The program counter is set to nnn plus the value of V0. It only uses V0, if other register is specified, it will still use V0
pub struct OptB {}
impl Executable for OptB {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        let r0 = chip.get_register_value(0);
        let eff_addr = specs.addr+r0 as u16;
        if Chip8::address_out_of_bounds(eff_addr) {
            Err("Address out of bounds in function call: OptB.execute() ".to_string())
        }
        else {
            chip.update_pc(Some(eff_addr));
            Ok(())
        }
    }
}