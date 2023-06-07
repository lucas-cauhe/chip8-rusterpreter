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
            _ => None// invalid nibble
        }
    }
    // LD VX VY
    fn execute_0(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_value = chip.get_register_value(specs.ry);
        chip.set_register_value(specs.rx, ry_value);
        chip.update_pc(None);
    }
    // OR VX VY
    fn execute_1(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, ry_val | rx_val); 
        chip.update_pc(None);   
    }
    // AND VX VY
    fn execute_2(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, ry_val & rx_val);  
        chip.update_pc(None);  
    }
    // XOR VX VY
    fn execute_3(&self, specs: OperationSpecs, chip: &mut Chip8) { 
        let ry_val = chip.get_register_value(specs.ry);
        let rx_val = chip.get_register_value(specs.rx);
        chip.set_register_value(specs.rx, ry_val ^ rx_val);
        chip.update_pc(None);    
    }
    // ADD VX VY
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
    // SUB VX VY
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
