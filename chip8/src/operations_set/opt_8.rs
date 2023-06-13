use super::operations_table::*;
use crate::Chip8;
pub struct Opt8 {}
impl Opt8 {
    pub fn select_execution(&self, specs: OperationSpecs, chip: &mut Chip8) -> Option<()> {
        match specs.nibble {
            0 => Some(self.execute_0(specs, chip)),
            1 => Some(self.execute_1(specs, chip)),
            2 => Some(self.execute_2(specs, chip)),
            3 => Some(self.execute_3(specs, chip)),
            4 => Some(self.execute_4(specs, chip)),
            5 => Some(self.execute_5(specs, chip)),
            6 => Some(self.execute_6(specs, chip)),
            7 => Some(self.execute_7(specs, chip)),
            0x0E => Some(self.execute_e(specs, chip)),
            _ => None // invalid nibble
        }
    }
    /// 8xy0 - LD Vx, Vy
    /// 
    ///Set Vx = Vy.
    ///
    ///Stores the value of register Vy in register Vx.
    fn execute_0(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_value = chip.get_register_value(specs.ry);
        chip.set_register_value(specs.rx, ry_value);
        chip.update_pc(None);
    }
    /// 8xy1 - OR Vx, Vy
    /// 
    /// Set Vx = Vx OR Vy.
    /// 
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. 
    /// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, 
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn execute_1(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, ry_val | rx_val); 
        chip.update_pc(None);   
    }

    /// 8xy2 - AND Vx, Vy
    /// 
    ///Set Vx = Vx AND Vy.
    ///
    ///Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. 
    ///A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, 
    ///then the same bit in the result is also 1. Otherwise, it is 0.
    fn execute_2(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, ry_val & rx_val);  
        chip.update_pc(None);  
    }

    /// 8xy3 - XOR Vx, Vy
    /// 
    /// Set Vx = Vx XOR Vy.
    /// 
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. 
    /// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, 
    /// then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn execute_3(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, ry_val ^ rx_val);
        chip.update_pc(None);    
    }
    
    /// 8xy4 - ADD Vx, Vy
    /// 
    /// Set Vx = Vx + Vy, set VF = carry.
    /// 
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. 
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn execute_4(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        if (rx_val as u16) +  (ry_val as u16) > 255 {
            chip.set_register_value(15, chip.get_register_value(15) | 0x02); // set overflow
        } else {
            chip.set_register_value(15, chip.get_register_value(15) & 0xFD); // unset overflow
        }
        chip.set_register_value(specs.rx, ry_val + rx_val);
        chip.update_pc(None);
    }

    /// 8xy5 - SUB Vx, Vy
    /// 
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// 
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn execute_5(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        if rx_val > ry_val {
            chip.set_register_value(15, chip.get_register_value(15) | 0x04); // set not borrow
        } else {
            chip.set_register_value(15, chip.get_register_value(15) & 0xFB); // unset not borrow
        }
        chip.set_register_value(specs.rx, rx_val - ry_val);
        chip.update_pc(None);
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// 
    /// Set Vx = Vx SHR 1.
    /// 
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn execute_6(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        if 0x01 & rx_val == 0x01 {
            chip.set_register_value(15, chip.get_register_value(15) | 0x08); // set LSB was 1
        } else {
            chip.set_register_value(15, chip.get_register_value(15) & 0xF7); // unset LSB was 1
        }
        chip.set_register_value(specs.rx, rx_val >> ry_val);
        chip.update_pc(None);    
    }

    /// 8xy7 - SUBN Vx, Vy
    /// 
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// 
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn execute_7(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        if ry_val > rx_val {
            chip.set_register_value(15, chip.get_register_value(15) | 0x04); // set not borrow
        } else {
            chip.set_register_value(15, chip.get_register_value(15) & 0xFB); // unset not borrow
        }
        chip.set_register_value(specs.rx, ry_val - rx_val);
        chip.update_pc(None);
    }

    /// 8xyE - SHL Vx {, Vy}
    /// 
    /// Set Vx = Vx SHL 1.
    /// 
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn execute_e(&self, specs: OperationSpecs, chip: &mut Chip8) { 
    let ry_val = chip.get_register_value(specs.ry);
    let rx_val = chip.get_register_value(specs.rx);
    if 0x80 & rx_val == 0x80 {
        chip.set_register_value(15, chip.get_register_value(15) | 0x10); // set MSB was 1
    } else {
        chip.set_register_value(15, chip.get_register_value(15) & 0xEF); // unset MSB was 1
    }
    chip.set_register_value(specs.rx, rx_val >> ry_val);
    chip.update_pc(None);    
}
}

impl Executable for Opt8 {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        match self.select_execution(specs, chip) {
            Some(()) => Ok(()),
            None => Err("Opt8 execution failed: ".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{operations_set::operations_table::OperationSpecs, chip8::Chip8};
    use super::Opt8;
    #[test]
    fn execute_0_test() {
        let specs = OperationSpecs {
            nibble: 0x00,
            addr: 0x0000,
            constant: 0,
            rx: 0x01,
            ry: 0x02
        };
        let mut chip = Chip8::new();
        chip.set_register_value(0x01, 0x01);
        chip.set_register_value(0x02, 0x02);
        Opt8::execute_0(&Opt8 {  }, specs, &mut chip);
        assert_eq!(0x02, chip.get_register_value(0x01));
    }
    #[test]
    fn execute_1_test() {
        let specs = OperationSpecs {
            nibble: 0x01,
            addr: 0x0000,
            constant: 0,
            rx: 0x01,
            ry: 0x02
        };
        let mut chip = Chip8::new();
        chip.set_register_value(0x01, 0x01);
        chip.set_register_value(0x02, 0x02);
        Opt8::execute_1(&Opt8 {  }, specs, &mut chip);
        assert_eq!(0x03, chip.get_register_value(0x01));
    }
}
