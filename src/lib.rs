

mod operations_set;
mod timers;
mod chip8;

use chip8::Chip8;

#[test]
fn create_chip() {
    let chip = Chip8::new();
    println!("{:?}", chip);
}
