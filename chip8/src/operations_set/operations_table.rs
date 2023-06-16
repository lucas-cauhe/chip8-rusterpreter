use crate::chip8::Chip8;
use super::{opt_1::Opt1, opt_2::Opt2, opt_3::Opt3, 
    opt_4::Opt4, opt_5::Opt5, opt_6::Opt6, 
    opt_7::Opt7, opt_8::Opt8, opt_9::Opt9,
    opt_a::OptA, opt_b::OptB, opt_c::OptC,
    opt_d::OptD, opt_f::OptF};

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
            0x10 => Some(Box::new(Opt1 {})),
            0x20 => Some(Box::new(Opt2 {})),
            0x30 => Some(Box::new(Opt3 {})),
            0x40 => Some(Box::new(Opt4 {})),
            0x50 => Some(Box::new(Opt5 {})),
            0x60 => Some(Box::new(Opt6 {})),
            0x70 => Some(Box::new(Opt7 {})),
            0x80 => Some(Box::new(Opt8 {})),
            0x90 => Some(Box::new(Opt9 {})),
            0xA0 => Some(Box::new(OptA {})),
            0xB0 => Some(Box::new(OptB {})),
            0xC0 => Some(Box::new(OptC {})),
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