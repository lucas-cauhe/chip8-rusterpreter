use std::sync::Arc;
use std::sync::Mutex;
use std::fs;
use std::sync::mpsc::Sender;

use crate::operations_set::operations_table::{OperationTab, OperationSpecs};
use crate::operations_set::operations_table::Executable;
use crate::timers::TimerThread;


const FIRST_REGISTER_ADDR: u16 = 0x010;
const STACK_INIT_ADDR: u16 = 0x020; // 16-bit aligned
const STACK_CANARY: u16 = 0x040; // if stack pointer tries to access a higher or equal address to this, raise exception
const PROGRAM_INIT_ADDR: u16 = 0x200;

// only for debugging purposes
static mut d_timer_done: i32 = 0;
static mut s_timer_done: i32 = 0;


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
            sound_timer: None
        }
    }

    pub fn set_register_value(&mut self, source: u8, value: u8) {
        self.memory[self.registers[source as usize] as usize] = value;
    }
    pub fn get_register_value(&mut self, source: u8) -> u8 {
        self.memory[self.registers[source as usize] as usize]
    }
    pub fn set_i_register_value(&mut self, value: u16) {
        self.i_register = value;
    }

    pub fn update_pc(&mut self, increment: Option<u16>) {
        self.pc = match increment {
            Some(addr) => addr,
            None => self.pc+0x0002
        }
    }

    // If the timer is already set and counting, it will ovewrite its value
    pub fn set_delay_timer(&mut self, val: u8) {
        
        if let Some((timer, ch)) = self.delay_timer.take() {
            // kill thread
            // there is the case when a thread has finished but the chip hasn't executed the next cycle so it won't have run the timer subroutine and the sender will send its message to nobody
            // That would give the failed to send message error
            // Therefore this condition is required
            if Arc::strong_count(&timer) == 2 {
                ch.send(()).expect("Failed to send message to delay timer thread");
            }
            
            
        }
        self.delay_timer = Some(TimerThread::launch(val));
    }

    pub fn set_sound_timer(&mut self, val: u8) {
        if let Some((timer, ch)) = self.sound_timer.take() {
            // kill thread
            if Arc::strong_count(&timer) == 2 {
                ch.send(()).expect("Failed to send message to delay timer thread");
            }
        }
        self.sound_timer = Some(TimerThread::launch(val));
    }

    
    pub fn load_program(&mut self, file: &str ) -> Result<(), String> {
        // parse file
        // TODO: transform addresses from hex to base 10
        // (fixed, returns error) check there are no weird registers being used (from G-)

        let code = fs::read_to_string(file).expect("I/O Error");
        
        
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

    fn parse_instruction(&self, inst: &[&str]) -> Result<u16, String> {
        
        let clean_reg = inst[1].replace(',', "");
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

            // retrieve certain operation
        let operation = OperationTab::fetch_operation((0xF0 & (next_opcode >> 8)) as u8).unwrap();// trait implementor to execute operation
        
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

        match self.delay_timer.take() {
            Some((timer, ch)) => {
                let t_left = timer.lock().unwrap();
                if t_left.timer == 0 {
                    // dispatch setter subroutine
                    unsafe { d_timer_done = 1; }
                } else {
                    drop(t_left);
                    self.delay_timer = Some((timer, ch))
                }
            },
            None => ()
        };

        match self.sound_timer.take() {
            Some((timer, ch)) => {
                let t_left = timer.lock().unwrap();
                if t_left.timer == 0 {
                    // dispatch setter subroutine
                    unsafe { s_timer_done = 1; }
                } else {
                    drop(t_left);
                    self.sound_timer = Some((timer, ch))
                }
            },
            None => ()
        };

        Ok(())
        
    }

}

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration, sync::Arc};

    use crate::chip8::{FIRST_REGISTER_ADDR, PROGRAM_INIT_ADDR, d_timer_done, s_timer_done};

    use super::Chip8;
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
            ["AND", "V1,", "V3"], 
            ["XOR", "VA,", "V8"],
            ["OR", "V7,", "V6"],
            ["OR", "VR,", "V6"]
        ].into_iter();
        let parsed_inst_1 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
        assert_eq!(Ok(0x8132), parsed_inst_1, "expected 0x8132, found: {:#06x}", parsed_inst_1.clone().unwrap());
        let parsed_inst_2 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
        assert_eq!(Ok(0x8A83), parsed_inst_2, "expected 0x8A83, found: {:#06x}", parsed_inst_2.clone().unwrap());
        let parsed_inst_3 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
        assert_eq!(Ok(0x8761), parsed_inst_3, "expected 0x8A83, found: {:#06x}", parsed_inst_3.clone().unwrap());
        let parsed_inst_4 = Chip8::parse_instruction(&test_chip, &test_instructions.next().unwrap());
        assert_eq!(matches!(parsed_inst_4, Err(_)), true);
    }

    #[test]
    fn load_program_1(){
        let mut chip = Chip8::new();
        chip.load_program("tests/mock_program.txt").unwrap();
        assert_eq!([
            0x81,
            0x20,
            0x8A,
            0xE0,
            0xA2,
            0x04,
            0x82,
            0x31,
            0x85,
            0x82,
            0x81,
            0x13
        ], chip.memory[512..524])
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
        chip.execute_cycle().unwrap();
        let d_t = chip.delay_timer.as_ref().unwrap();
        let d_tlck = d_t.0.lock().unwrap();
        assert_eq!(2, d_tlck.timer);
        assert_eq!(2, Arc::strong_count(&d_t.0));
        drop(d_tlck);
        thread::sleep(Duration::new(1, 0));
        chip.execute_cycle().unwrap();
        thread::sleep(Duration::new(1, 0));
        chip.execute_cycle().unwrap();
        thread::sleep(Duration::new(1, 0));
        unsafe {
            assert_eq!(d_timer_done, 1, "Delay timer not finished yet");
            assert_eq!(s_timer_done, 1, "Sound timer not finished yet");
        }
        
        
    }
}