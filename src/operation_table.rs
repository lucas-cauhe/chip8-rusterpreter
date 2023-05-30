use crate::chip8::OperationSpecs;



pub struct OperationTab { }
impl OperationTab {
    pub fn fetch_operation(code: u8) -> Option<impl Executable> {
        match code {
            0 => todo!(),
            8 => Some(Opt8 {}),
            _ => None
        }
    }
}


pub trait Executable {
    // - Execute custom operation
    // - Modify PC
    fn execute(&self, specs: OperationSpecs) -> Result<(), String>;
}

struct Opt8 {}
impl Opt8 {
    pub fn select_execution(&self, specs: OperationSpecs) -> Option<()> {
        match specs.nibble {
            0 => Some(self.execute_0(specs)),
            1 => Some(self.execute_1(specs)),
            // ...
            _ => None// invalid nibble
        }
    }
    
    fn execute_0(&self, specs: OperationSpecs) { todo!() }
    fn execute_1(&self, specs: OperationSpecs) { todo!() }
    //...
}

impl Executable for Opt8 {
    fn execute(&self, specs: OperationSpecs) -> Result<(), String> {
        match self.select_execution(specs) {
            Some(()) => Ok(()),
            None => Err("Opt8 execution failed: ".to_string())
        }
    }
}
