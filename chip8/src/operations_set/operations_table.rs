use crate::chip8::Chip8;
use super::{opt8::Opt8, opt_f::OptF, opt_d::OptD};

pub struct OperationSpecs {
    pub nibble: u8,
    pub addr: u16,
    pub constant: u8,
    pub rx: u8,
    pub ry: u8
}

pub struct OperationTab { }
impl OperationTab {
    pub fn fetch_operation(code: u8) -> Option<Box<dyn Executable>> {
        match code {
            0 => todo!(),
            0x80 => Some(Box::new(Opt8 {})),
            0xD0 => Some(Box::new(OptD {})),
            0xF0 => Some(Box::new(OptF {})),
            _ => None
        }
    }
}


pub trait Executable {
    // - Execute custom operation
    // - Modify PC
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String>;
}

// special instructions

pub struct Ret { }
pub struct Cls { }

impl Executable for Ret {
    fn execute(&self, _specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        chip.leave_subroutine();
        chip.update_pc(None);
        Ok(())
    }
}
impl Executable for Cls {
    fn execute(&self, _specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        chip.clear_display();
        chip.update_pc(None);
        Ok(())
    }
}