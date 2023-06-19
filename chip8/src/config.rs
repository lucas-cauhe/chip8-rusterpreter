
use serde::{Serialize, Deserialize};
use std::fs;
use serde_yaml;
use super::chip8::ChipConfig;

// chip constants

const FIRST_REGISTER_ADDR: u16 = 0x010;
const STACK_INIT_ADDR: u16 = 0x020; // 16-bit aligned
const STACK_CANARY: u16 = 0x040; // if stack pointer tries to access a higher or equal address to this, raise exception
const PROGRAM_INIT_ADDR: u16 = 0x200;
pub const RTI_DEFAULT_ADDR: u16 = 0x600;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;
const EOP_OPT_CODE: u16 = 0x0000;

// display constants

const WINDOW_WIDTH: u32 = 768;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub window_width: u32,
    pub window_height: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    chip: ChipConfig,
    display: DisplayConfig
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ConfigParserEnum {
    Chip {
        first_register_addr: Option<u16>,
        stack_init_addr: Option<u16>,
        stack_canary: Option<u16>,
        program_init: Option<u16>,
        rti_default_addr: Option<u16>,
        display_height: Option<usize>,
        display_width: Option<usize>,
        eop_opt_code: Option<u16>
    },
    Display {
        window_height: Option<u32>,
        window_width: Option<u32>
    }
}

fn find_config_file() -> Option<String> {
    let mut file = None;
    let args = std::env::args().collect::<Vec<String>>();
    for (arg_no, arg) in args.iter().enumerate() {
        if arg == "--config" {
            file = Some(args[arg_no+1].clone());
        }
    }
    file 
}

pub fn parse_chip() -> ChipConfig {

    let mut chip_conf: Option<ConfigParserEnum> = None;
    if let Some(file) = find_config_file() {  
        let file = fs::read_to_string(file).unwrap();
        let deserialized_chip_config: Vec<ConfigParserEnum> = serde_yaml::from_str(&file).unwrap();
        if deserialized_chip_config.len() > 0 {
            if let ConfigParserEnum::Chip { .. } = deserialized_chip_config[0] {
                chip_conf = Some(deserialized_chip_config[0].clone());
            }
            else if let ConfigParserEnum::Chip { .. } = deserialized_chip_config[1] {
                chip_conf = Some(deserialized_chip_config[1].clone());
            }
        }
    }
    ChipConfig {
        first_register_addr: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{first_register_addr, ..} = chip { 
                
                    match first_register_addr {
                        Some(target) => target.clone(),
                        None => FIRST_REGISTER_ADDR
                    }
                } 
                else {
                    FIRST_REGISTER_ADDR
                }
            }, 
            None => FIRST_REGISTER_ADDR 
        },
        display_height: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{display_height, ..} = chip { 
                    match display_height {
                        Some(target) => target.clone(),
                        None => DISPLAY_HEIGHT
                    }
                } 
                else {
                    DISPLAY_HEIGHT
                }
            }, 
            None => DISPLAY_HEIGHT 
        },
        display_width: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{display_width, ..} = chip { 
                    match display_width {
                        Some(target) => target.clone(),
                        None => DISPLAY_WIDTH
                    }
                }
                else {
                    DISPLAY_WIDTH
                } 
            }, 
            None => DISPLAY_WIDTH 
        },
        program_init: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{program_init, ..} = chip { 
                    match program_init {
                        Some(target) => target.clone(),
                        None => PROGRAM_INIT_ADDR
                    }
                } 
                else {
                    PROGRAM_INIT_ADDR
                }
            }, 
            None => PROGRAM_INIT_ADDR 
        },
        rti_default_addr: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{rti_default_addr, ..} = chip { 
                    match rti_default_addr {
                        Some(target) => target.clone(),
                        None => RTI_DEFAULT_ADDR
                    }
                } 
                else {
                    RTI_DEFAULT_ADDR
                }
            }, 
            None => RTI_DEFAULT_ADDR 
        },
        eop_opt_code: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{eop_opt_code, ..} = chip { 
                    match eop_opt_code {
                        Some(target) => target.clone(),
                        None => EOP_OPT_CODE
                    }
                }
                else {
                    EOP_OPT_CODE
                }
            }, 
            None => EOP_OPT_CODE 
        },
        stack_canary: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{stack_canary, ..} = chip { 
                    match stack_canary {
                        Some(target) => target.clone(),
                        None => STACK_CANARY
                    }
                } 
                else {
                    STACK_CANARY
                }
            }, 
            None => STACK_CANARY 
        },
        stack_init_addr: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Chip{stack_init_addr, ..} = chip { 
                    match stack_init_addr {
                        Some(target) => target.clone(),
                        None => STACK_INIT_ADDR
                    }
                }
                else {
                    STACK_INIT_ADDR
                }
            }, 
            None => STACK_INIT_ADDR 
        }
    }
}


pub fn parse_display() -> DisplayConfig {
    let file = fs::read_to_string("chip8-config.yaml").unwrap();
    let deserialized_chip_config: Vec<ConfigParserEnum> = serde_yaml::from_str(&file).unwrap();
    let mut chip_conf: Option<ConfigParserEnum> = None;
    if deserialized_chip_config.len() > 0 {
        if let ConfigParserEnum::Chip { .. } = deserialized_chip_config[0] {
            chip_conf = Some(deserialized_chip_config[0].clone());
        }
        else if let ConfigParserEnum::Chip { .. } = deserialized_chip_config[1] {
            chip_conf = Some(deserialized_chip_config[1].clone());
        }
    }
    DisplayConfig {
        window_height: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Display { window_height, .. } = chip { 
                
                    match window_height {
                        Some(target) => target.clone(),
                        None => WINDOW_HEIGHT
                    }
                } 
                else {
                    WINDOW_HEIGHT
                }
            }, 
            None => WINDOW_HEIGHT 
        },
        window_width: match chip_conf { 
            Some(ref chip)  =>  { 
                if let ConfigParserEnum::Display { window_width, .. } = chip { 
                
                    match window_width {
                        Some(target) => target.clone(),
                        None => WINDOW_WIDTH
                    }
                } 
                else {
                    WINDOW_WIDTH
                }
            }, 
            None => WINDOW_WIDTH 
        },
    }
}