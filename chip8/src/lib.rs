
pub mod operations_set;
pub mod timers;
pub mod chip8;
pub mod config;

use serde::{Serialize, Deserialize};
use chip8::{Chip8, ChipConfig};

#[test]
fn create_chip() {
    let chip = Chip8::new();
    println!("{:?}", chip);
}