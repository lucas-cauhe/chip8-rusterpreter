
The chip8-rusterpreter is an emulator for chip8 language written in rust, which comes along with a debugger.

## Run debugger with
````
cargo run -p debugger -- <chip8-file>
````

## Run emulator with
````
cargo run -p chip8 -- <chip8-file>
````

## Run tests with
````
cargo test -p <package>
````

You can define memory address in hexadecimal form now

## Customize chip's and display's parameters through .yaml config file

````
cargo run -p chip8 -- <chip8-file> --config path/to/file.yaml
````
All parameters not specified in the config file (if a config file is passed) will be set to their default value

# Example of config file

````
- !Chip
  rti_default_addr: 0x700
  program_init: 0x500

- !Display
  window_height: 300
  window_width: 400

````

# Available Parameters

- first_register_addr: Where are registers be placed in memory
- stack_init_addr: Where is the stack placed in memory
- stack_canary: stack_init_addr + stack_size + 2
- program_init: Where program memory space starts
- rti_default_addr: Where should RTIs be placed
- display_height: 32 by default
- display_width: 64 by default
- eop_opt_code: operation code that exits the running program (0 by default)