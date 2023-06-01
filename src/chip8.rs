use std::sync::Arc;
use std::sync::Mutex;
use std::fs;
use std::sync::mpsc::Sender;

use crate::operations_set::operations_table::Executable;
use crate::operations_set::operations_table::{OperationTab, OperationSpecs, Ret, Cls};
use crate::timers::TimerThread;


const FIRST_REGISTER_ADDR: u16 = 0x010;
const STACK_INIT_ADDR: u16 = 0x020; // 16-bit aligned
const STACK_CANARY: u16 = 0x040; // if stack pointer tries to access a higher or equal address to this, raise exception
const PROGRAM_INIT_ADDR: u16 = 0x200;
pub const RTI_DEFAULT_ADDR: u16 = 0x600;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;


// only for debugging purposes


#[derive(Debug)]
pub struct Chip8 {
    memory: Vec<u8>,
    registers: Vec<u16>, // list of memory mapped registers
    stack: Vec<u16>,
    i_register: u16,
    pc: u16,
    sp: u8,
    delay_timer: Option<(Arc<Mutex<TimerThread>>, Sender<()>)>,
    sound_timer: Option<(Arc<Mutex<TimerThread>>, Sender<()>)>,
    gfx: Vec<Vec<u8>>
}

impl Chip8 {
    pub fn new() -> Self {
        
        let registers: Vec<u16> = {
            let mut inter = Vec::new();
            for i in 0..16 {
               inter.push(FIRST_REGISTER_ADDR + (i as u16));
            }
            inter
        };

        let stack: Vec<u16> = {
            let mut inter = Vec::new();
            for i in 0..16 {
               inter.push(STACK_INIT_ADDR + (2*i as u16));
            }
            inter
        };

        Chip8 {
            memory: Vec::from([0; 4096]),
            registers,
            stack,
            i_register: 0x000,
            pc: PROGRAM_INIT_ADDR,
            sp: 0x00, // access by STACK_INIT_ADDR + sp in memory
            delay_timer: None,
            sound_timer: None,
            gfx: vec![vec![0_u8; 8]; 32]
        }
    }

    pub fn set_register_value(&mut self, source: u8, value: u8) {
        self.memory[self.registers[source as usize] as usize] = value;
    }
    pub fn get_register_value(&self, source: u8) -> u8 {
        self.memory[self.registers[source as usize] as usize]
    }
    pub fn set_i_register_value(&mut self, value: u16) {
        self.i_register = value;
    }
    pub fn leave_subroutine(&mut self) {
        self.sp -= 1;
        self.pc = (self.memory[self.stack[self.sp as usize] as usize] as u16) << 8 | self.memory[self.stack[self.sp as usize+1] as usize] as u16;
    }
    pub fn clear_display(&mut self) {
        self.gfx = vec![vec![0_u8; 8]; 32];
    }

    pub fn update_pc(&mut self, increment: Option<u16>) {
        self.pc = match increment {
            Some(addr) => addr,
            None => self.pc+0x0002
        }
    }
    pub fn load_i_address_value(&self, offset: usize) -> u8 {
        self.memory[self.i_register as usize + offset]
    }
    pub fn get_gfx_sprite(&self, coords: (u8, u8), offset: usize) -> u64 {
        // take into acount the possible cyclic representation
        let target_row = (coords.0 as usize + offset) % DISPLAY_HEIGHT;
        let target_col = coords.1 as usize % DISPLAY_WIDTH;
        let target_byte_mask = {
            let mut mask = 0x00000000;
            for bit in 0..8 {
                mask |= 1 << (DISPLAY_WIDTH-1-((target_col+bit) % DISPLAY_WIDTH));
            }
            mask
        };
        let row_gfx_value = u64::from_be_bytes(self.gfx[target_row].clone().try_into().unwrap());
        target_byte_mask & row_gfx_value
    }

    pub fn set_gfx_sprite(&mut self, coords: (u8, u8), offset: usize, sprite: u8) {
        let target_row = (coords.0 as usize + offset) % DISPLAY_HEIGHT;
        let _target_col = coords.1 as usize % DISPLAY_WIDTH;
        let prev_sprite = self.get_gfx_sprite(coords, offset);
        let xor_sprite = (sprite as u64).rotate_right(8+coords.1 as u32) ^ prev_sprite;

        // check collisions
        if prev_sprite != (prev_sprite & xor_sprite) {
            self.memory[self.registers[15] as usize] |= 0x01; 
        }
        // draw it in gfx
        for i in 0..8 {
            self.gfx[target_row][i] = (((0xFF as u64).rotate_right((8*(i+1)) as u32) & xor_sprite) >> (DISPLAY_WIDTH-((i+1)*8))) as u8;
        }
    }

    // If the timer is already set and counting, it will ovewrite its value
    pub fn set_delay_timer(&mut self, val: u8, rti: Option<u16>) {
        
        if let Some((timer, ch)) = self.delay_timer.take() {
            // kill thread
            // there is the case when a thread has finished but the chip hasn't executed the next cycle so it won't have run the timer subroutine and the sender will send its message to nobody
            // That would give the failed to send message error
            // Hence this condition is required
            if Arc::strong_count(&timer) == 2 {
                ch.send(()).expect("Failed to send message to delay timer thread");
            }
            
            
        }
        self.delay_timer = Some(TimerThread::launch(val, rti));
    }

    pub fn set_sound_timer(&mut self, val: u8, rti: Option<u16>) {
        if let Some((timer, ch)) = self.sound_timer.take() {
            // kill thread
            if Arc::strong_count(&timer) == 2 {
                ch.send(()).expect("Failed to send message to delay timer thread");
            }
        }
        self.sound_timer = Some(TimerThread::launch(val, rti));
    }

    
    pub fn load_program(&mut self, file: &str ) -> Result<(), String> {
        // parse file
        // (fixed, returns error) check there are no weird registers being used (from G-)

        let mut code = fs::read_to_string(file).expect("I/O Error");
        // only handles directives, not labels
        //self.parse_directives( &mut code);
        
        let mut inst_buff: Vec<u8> = Vec::new();
        for line in code.lines() {
            let inst: Vec<&str> = line.split(' ').collect();
            let parsed_inst: u16 = self.parse_instruction(&inst)?;
            inst_buff.push(((0xFF00 & parsed_inst) >> 8) as u8); // big-endian
            inst_buff.push((0x00FF & parsed_inst) as u8);
        }

        for (i, inst) in inst_buff.into_iter().enumerate() {
            self.memory[(PROGRAM_INIT_ADDR as usize)+i] = inst;
        }
        Ok(())
    }

    /* fn parse_directives(&mut self, text: &mut String) {
        loop {
            let previous_code_to_directive = text.find("!").unwrap();
        }
    } */

    fn parse_instruction(&self, inst: &[&str]) -> Result<u16, String> {
        
        let clean_reg = if inst.len() > 1 { inst[1].replace(',', "") } else { "".to_string() };
        match inst[0] {
            "LD" => {
                match clean_reg.as_str() {
                    "I" => {
                        let addr_mask = 0x0FFF & (inst[2].parse::<u16>().unwrap());
                        // check it is not accessing out of bounds address
                        if (0xFE00 & addr_mask) == 0 {
                            Err(format!("Address out of bounds: {:#06x}", addr_mask))
                        } else {
                            Ok(0xA000 | addr_mask)
                        }
                    },
                    "DT" => {
                        let vx = inst[2].chars().nth(1).unwrap().to_digit(16).unwrap() as u16;
                        let vx_mask = 0x0F00 & (vx << 8);
                        Ok(0xF015 | vx_mask)
                    },
                    _ => { // it is a common register
                        match self.parse_common_registers(&clean_reg, inst[2]) {
                            Some((regx, regy)) => Ok(0x8000 | (regx << 8) | (regy << 4)),
                            None => Err(format!("Error parsing instruction: {:?}", inst))
                        }
                    }
                }
            },
            "OR" => {
                match self.parse_common_registers(&clean_reg, inst[2]) {
                    Some((regx, regy)) => Ok(0x8001 | (regx << 8) | (regy << 4)),
                    None => Err(format!("Error parsing instruction: {:?}", inst))
                }
                
            },
            "AND" => {
                match self.parse_common_registers(&clean_reg, inst[2]) {
                    Some((regx, regy)) => Ok(0x8002 | (regx << 8) | (regy << 4)),
                    None => Err(format!("Error parsing instruction: {:?}", inst))
                }
            },
            "XOR" => {
                match self.parse_common_registers(&clean_reg, inst[2]) {
                    Some((regx, regy)) => Ok(0x8003 | (regx << 8) | (regy << 4)),
                    None => Err(format!("Error parsing instruction: {:?}", inst))
                }
            },
            /* "ADD" => {
                let (regx, regy) = self.parse_common_registers(&clean_reg, inst[2]);
                0x8004 | (regx << 8) | (regy << 8)
            }, */
            "DRW" => {
                match self.parse_common_registers(&clean_reg, inst[2].replace(',', "").as_str()) {
                    Some((regx, regy)) => Ok(0xD000 | (regx << 8) | (regy << 4) | (0x000F & inst[3].parse::<u16>().unwrap())),
                    None => Err(format!("Error parsing instruction: {:?}", inst))
                }
            },
            "RET" => Ok(0x00EE),
            "CLS" => Ok(0x00E0),
            _ => Err("Undefined Instruction".to_string()) // undefined instruction
        }

    }

    fn parse_common_registers(&self, rx: &str, ry: &str) -> Option<(u16,u16)> {
        
        if let (Some(rx_value), Some(ry_value)) = (rx.chars().nth(1).unwrap().to_digit(16), ry.chars().nth(1).unwrap().to_digit(16))
        {
            Some((
                rx_value as u16, ry_value as u16
            ))
        }
        else { None }
    }

    pub fn execute_cycle(&mut self) -> Result<(), String> {
        // Fetch next opcode

        let next_opcode: u16 = ((self.memory[(self.pc as usize)] as u16) << 8) | self.memory[(self.pc as usize)+1] as u16;

        // Decode opcode

        // special operations
        let operation: Box<dyn Executable>  = match next_opcode {
            0x00EE => Box::new(Ret { }),
            0x00E0 => Box::new(Cls { }),
            _ => OperationTab::fetch_operation((0xF0 & (next_opcode >> 8)) as u8).unwrap()
        };
        
        let nibble: u8 = (0x000F & next_opcode) as u8; // operation code (if has one)
        let addr: u16 = 0x0FFF & next_opcode;
        let constant: u8 = (0x00FF & next_opcode) as u8;
        let rx: u8 = (0x0F & (next_opcode >> 8)) as u8;
        let ry: u8 = (0x0F & (next_opcode >> 4)) as u8;

        let opt_specs = OperationSpecs {
            nibble, addr, constant, rx, ry
        };

        // Execute

            // execute retrieved operation with chip parameters
        operation.execute(opt_specs, self)?;

        // Update timers
        // Perhaps handling these as interruptions would be better

        let delay_t_called = match self.delay_timer.take() {
            Some((timer, ch)) => {
                let t_left = timer.lock().unwrap();
                if t_left.timer == 0 {
                    // dispatch setter subroutine
                    self.call_subroutine(t_left.rti)?;
                    true
                } else {
                    drop(t_left);
                    self.delay_timer = Some((timer, ch));
                    false
                }
            },
            None => false
        };

        match self.sound_timer.take() {
            // the sound timer is set and delay timer hasn't called its subroutine
            Some((timer, ch)) => {
                let t_left = timer.lock().unwrap();
                if t_left.timer == 0  && !delay_t_called {
                    // dispatch setter subroutine
                    self.call_subroutine(t_left.rti)?;
                } else {
                    drop(t_left);
                    self.sound_timer = Some((timer, ch))
                }
            },
            _ => ()
        };

        Ok(())
        
    }

    fn call_subroutine(&mut self, addr: u16) -> Result<(), String>{
        // check stack overflow
        let next_sp = STACK_INIT_ADDR + (self.sp as u16)*2;
        if next_sp == STACK_CANARY {
            Err("Reached stack canary: buffer overflow".to_string())
        }
        else {
            // store pc where sp point to
            self.memory[next_sp as usize] = ((self.pc >> 8) & 0x00FF) as u8;
            self.memory[next_sp as usize+1] = (self.pc & 0x00FF) as u8;
            // increment sp
            self.sp += 1;

            // modify pc to subroutine's address
            self.pc = addr;

            Ok(())
        }
        
    }

}

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration, sync::Arc};

    use crate::chip8::{FIRST_REGISTER_ADDR, PROGRAM_INIT_ADDR, RTI_DEFAULT_ADDR, STACK_INIT_ADDR};

    use super::Chip8;
    mod parsing_tests {
        use super::*;
        #[test]
        fn parse_common_registers_test() {
            let test_chip = Chip8::new();
            let parsed_tup_1 = Chip8::parse_common_registers(&test_chip, "V5", "V6");
            assert_eq!(Some((0x0005, 0x0006)), parsed_tup_1);
            let parsed_tup_2 = Chip8::parse_common_registers(&test_chip, "VA", "V3");
            assert_eq!(Some((0x000A, 0x0003)), parsed_tup_2);
            let parsed_tup_3 = Chip8::parse_common_registers(&test_chip, "VF", "VE");
            assert_eq!(Some((0x000F, 0x000E)), parsed_tup_3);
            let parsed_tup_4 = Chip8::parse_common_registers(&test_chip, "VR", "VE");
            assert_eq!(None, parsed_tup_4);
        }

        #[test]
        fn parse_instruction_test() {
            let test_chip = Chip8::new();
            let mut test_instructions = [
                vec!["AND", "V1,", "V3"], 
                vec!["XOR", "VA,", "V8"],
                vec!["OR", "V7,", "V6"],
                vec!["OR", "VR,", "V6"],
                vec!["DRW", "V1,", "V3,", "13"],
                vec!["RET"],
                vec!["CLS"]
            ].into_iter();
            let parsed_inst_1 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(Ok(0x8132), parsed_inst_1, "expected 0x8132, found: {:#06x}", parsed_inst_1.clone().unwrap());
            let parsed_inst_2 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(Ok(0x8A83), parsed_inst_2, "expected 0x8A83, found: {:#06x}", parsed_inst_2.clone().unwrap());
            let parsed_inst_3 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(Ok(0x8761), parsed_inst_3, "expected 0x8A83, found: {:#06x}", parsed_inst_3.clone().unwrap());
            let parsed_inst_4 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(matches!(parsed_inst_4, Err(_)), true);
            let parsed_inst_5 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(Ok(0xD13D), parsed_inst_5,"expected 0xD13D, found: {:#06x}", parsed_inst_5.clone().unwrap() );
            let parsed_inst_6 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(Ok(0x00EE), parsed_inst_6,"expected 0x00EE, found: {:#06x}", parsed_inst_6.clone().unwrap() );
            let parsed_inst_7 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
            assert_eq!(Ok(0x00E0), parsed_inst_7,"expected 0x00E0, found: {:#06x}", parsed_inst_7.clone().unwrap() );
        }

        #[test]
        fn load_program_1(){
            let mut chip = Chip8::new();
            chip.load_program("tests/mock_program.txt").unwrap();
            assert_eq!([
                0x81, 0x20,
                0x8A, 0xE0, 
                0xA2, 0x04,
                0x82, 0x31,
                0x85, 0x82,
                0x81, 0x13
            ], chip.memory[512..524])
        }
    }

    mod execution_tests {
        use super::*;
        #[test]
        fn call_subroutine_test() {
            let mut chip = Chip8::new();
            chip.call_subroutine(RTI_DEFAULT_ADDR).unwrap();
            assert_eq!(chip.pc, RTI_DEFAULT_ADDR);
            // test stack storage
            assert_eq!(chip.memory[STACK_INIT_ADDR as usize..STACK_INIT_ADDR as usize+2], [0x02, 0x00]);
            assert_eq!(chip.sp, 1);
            // return from subroutine
            chip.leave_subroutine();
            assert_eq!(chip.pc, 0x0200);
            assert_eq!(chip.sp, 0);
        }

        #[test]
        fn get_gfx_sprite_test() {
            let mut chip = Chip8::new();
            // set everything up
            chip.gfx[0] = vec![0x13, 0x14, 0x15, 0x16, 0x00, 0x00, 0x00, 0x00];
            chip.gfx[1] = vec![0x17, 0x18, 0x19, 0x20, 0x00, 0x00, 0x00, 0x00];
            chip.gfx[2] = vec![0x21, 0x22, 0x23, 0x24, 0x00, 0x00, 0x00, 0x00];
            // test
            assert_eq!(chip.get_gfx_sprite((0,0), 0), 0x1300000000000000);
            assert_eq!(chip.get_gfx_sprite((0,0), 1), 0x1700000000000000);
            assert_eq!(chip.get_gfx_sprite((0,0), 2), 0x2100000000000000);
        }

        #[test]
        fn set_gfx_sprite_test() {
            let mut chip = Chip8::new();

            chip.gfx[0] = vec![0x13, 0x14, 0x15, 0x16, 0x00, 0x00, 0x00, 0x00];
            chip.gfx[1] = vec![0x17, 0x18, 0x19, 0x20, 0x00, 0x00, 0x00, 0x00];
            chip.gfx[2] = vec![0x21, 0x22, 0x23, 0x24, 0x00, 0x00, 0x00, 0x00];

            assert_eq!(chip.get_gfx_sprite((0,4), 0), 0x0310000000000000);

            chip.set_gfx_sprite((0,0), 0, 0x03);
            assert!(chip.memory[chip.registers[15] as usize] & 0x01 == 0x01); // check collision is activated
            chip.set_gfx_sprite((0,0), 1, 0x05);
            chip.set_gfx_sprite((0,0),2, 0x07);

            assert_eq!(chip.get_gfx_sprite((0,0), 0), 0x1000000000000000); // 0x13 xor 0x03 = 0x10
            assert_eq!(chip.get_gfx_sprite((0,0), 1), 0x1200000000000000); // 0x17 xor 0x05 = 0x12
            assert_eq!(chip.get_gfx_sprite((0,0), 2), 0x2600000000000000); // 0x21 xor 0x07 = 0x26
        }

        // TIMERS TEST
        #[test]
        fn execute_program_1() {
            let test_prog: Vec<u8> = vec![0xF3, 0x15, 0xF4, 0x18, 0x81, 0x11];
            let mut chip = Chip8::new();
            // set register values
            chip.memory[(FIRST_REGISTER_ADDR as usize) + 3] = 0x02;
            chip.memory[(FIRST_REGISTER_ADDR as usize) + 4] = 0x02;
            for i in 0..test_prog.len() {
                chip.memory[ (PROGRAM_INIT_ADDR as usize)+ i] = test_prog[i];
            }
            chip.memory[RTI_DEFAULT_ADDR as usize] = 0x81;
            chip.memory[RTI_DEFAULT_ADDR as usize+1] = 0x11;

            chip.execute_cycle().unwrap();

            let d_t = chip.delay_timer.as_ref().unwrap();
            let d_tlck = d_t.0.lock().unwrap();
            assert_eq!(2, d_tlck.timer);
            assert_eq!(2, Arc::strong_count(&d_t.0));
            drop(d_tlck);
            thread::sleep(Duration::new(1, 0));

            chip.execute_cycle().unwrap();

            // delay timer's subroutine
            assert_eq!(RTI_DEFAULT_ADDR, chip.pc);
            thread::sleep(Duration::new(1, 0));

            chip.execute_cycle().unwrap();

            // sound timer's subroutine
            assert_eq!(RTI_DEFAULT_ADDR, chip.pc);
            thread::sleep(Duration::new(1, 0));
            
        }
    }   


}