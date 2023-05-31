use crate::chip8::Chip8;
use super::{opt8::Opt8, opt_f::OptF};

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