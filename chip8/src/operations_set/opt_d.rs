use super::operations_table::*;
use crate::Chip8;

/// Dxyn - DRW Vx, Vy, nibble
///
/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
///
/// The interpreter reads n bytes from memory, starting at the address stored in I. 
///
/// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. 
///
/// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. 
///
/// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. 
///
/// See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
pub struct OptD {}


impl Executable for OptD {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {

        // take coordinates values
        let coord = (chip.get_register_value(specs.rx), chip.get_register_value(specs.ry));

        for i in 0..specs.nibble as usize {
            let sprite = chip.load_i_address_value(i);
            chip.set_gfx_sprite(coord, i, sprite);
        }
        
        // activate draw_flag
        let mut flag_reg = chip.get_register_value(0x0F);
        flag_reg = flag_reg | 0x80;
        chip.set_register_value(0x0F, flag_reg);
        chip.update_pc(None);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{operations_set::operations_table::OperationSpecs, chip8::Chip8};

    
    #[test]
    fn opt_d_execution() {
        // DRW V1, V2, 3
        let specs = OperationSpecs {
            nibble: 0x03,
            addr: 0x0000,
            constant: 0,
            rx: 0x01,
            ry: 0x02
        };
        let mut chip = Chip8::new();
        // (0,0) coordinates
        chip.set_register_value(0x01, 0x00);
        chip.set_register_value(0x02, 0x00);
        chip.set_i_register_value(0x600);
        // Three store operations for the nibbles
        // TODO
        //OptD::execute(&OptD {  }, specs, &mut chip).unwrap();
    }
}