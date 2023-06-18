use super::operations_table::*;
use crate::Chip8;
use rand;


/// Cxkk - RND Vx, byte
/// 
/// Set Vx = random byte AND kk.
/// 
/// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. 
/// The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub struct OptC {}
impl Executable for OptC {
    fn execute(&self, specs: OperationSpecs, chip: &mut Chip8) -> Result<(), String> {
        let rand_no = rand::random::<u8>();
        chip.set_register_value(specs.rx, rand_no & specs.constant);
        Ok(())
    }
}