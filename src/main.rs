pub mod vm;
pub mod assembler;

use std::io;
use std::io::prelude::*;

use crate::vm::VM;
use crate::vm::instructions::Opcode;

const COMMAND_PREFIX: char = '!';

pub static BANNER: &str = "This is a thing. Type stuff. Yay.";
pub static PROMPT: &str = ">>> ";

fn main() {
    println!("{}", BANNER);

    let mut vm = VM::default();

    'main: loop {
        print!("Registers: ");

        for register in &vm.registers {
            print!("{:#04x} ", register);
        }

        println!();

        println!("Remainder: {}          Equality Flag: {}", vm.remainder, vm.equal_flag);

        print!("{}", PROMPT);
        
        io::stdout().flush().expect("Could not flush stdout");

        let stdin = io::stdin();
        let input = &mut String::new();
        
        stdin.read_line(input).expect("Unable to read line.");

        if input.starts_with(COMMAND_PREFIX) {
            let mut iter = input.chars();

            // Skip the prefix
            iter.next();

            // Split the command an arguments
            let parts: Vec<&str> = iter.as_str().split_whitespace().collect();

            // Helper variables
            let command = parts[0].to_lowercase();
            let _args = &parts[1..];

            if command == "quit" {
                break 'main;
            } else {
                println!("Unknown command: {}", command);
            }
        } else {
            let tokens: Vec<&str> = input.split_whitespace().collect();
    
            let op = Opcode::from(tokens[0].to_uppercase()).byte();
    
            print!("{:#04x} ", op);
            
            let mut bytes: Vec<u8> = Vec::new();
    
            for arg in &tokens[1..] {
                let result = u8::from_str_radix(&arg, 16);
    
                match result {
                    Err(e) => {
                        println!("<{}>", e);
                        continue 'main;
                    },
                    Ok(byte) => {
                        print!("<{:#04x}> ", byte);
    
                        bytes.append(&mut vec![ byte ]);
                    }
                }
            }
    
            vm.program.append(&mut vec![ op ]);
    
            vm.program.append(&mut bytes);
    
            println!();
    
            vm.run_once();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time;
    
    use crate::vm::VM;
    use crate::vm::instructions::Opcode;

    #[test]
    pub fn fib() {
        const TIMES: u16 = 65535;

        const MAX_ITERATIONS: u8 = 45;

        let mut totals: Vec<u128> = vec![];
        
        for _i in 0..TIMES {
            let before = time::Instant::now();

            let mut last = 0;
            let mut curr = 1;

            for _i in 1..MAX_ITERATIONS {
                let sum = last + curr;
                last = curr;
                curr = sum;
            }
            
            totals.push(before.elapsed().as_nanos());
        }
        
        println!("Rust took: {:.2?}ns", (totals.iter().sum::<u128>() / totals.len() as u128));

        let mut totals: Vec<u128> = vec![];
        
        for _i in 0..TIMES {
            let mut test_vm = VM::default();

            test_vm.program = vec![
                Opcode::Load.byte(), 0, 0, 1,   // Load $0 with 1
                Opcode::JumpForward.byte(), 0,  // Jump forward $0 bytes
                Opcode::Halt.byte(),

                Opcode::Load.byte(), 0, 0, 6,   // Load $0 with 6: byte of the halt instruction

                Opcode::Load.byte(), 1, 0, 0,   // Load $1 with 0
                Opcode::Load.byte(), 2, 0, MAX_ITERATIONS - 1,

                Opcode::Load.byte(), 3, 0, 0,   // Load $3 with 0
                Opcode::Load.byte(), 4, 0, 1,   // Load $4 with 1
                
                Opcode::Load.byte(), 6, 0, 31,  // Load $6 with 32: the start of the iteration

                // Start of fib iterations
                Opcode::Equal.byte(), 1, 2,     // Check if $1 (current) and $2 (max) are equal
                Opcode::JumpIfEqual.byte(), 0,  // Jump to byte $0 if equal flag is set

                Opcode::Increment.byte(), 1,    // Increment $1
                Opcode::Add.byte(), 3, 4, 5,    // $3 + $4 = $5
                Opcode::Move.byte(), 4, 3,      // $3 = $4
                Opcode::Move.byte(), 5, 4,      // $4 = $5
                
                Opcode::Jump.byte(), 6,         // Jump to byte $6
            ];

            let before = time::Instant::now();
            
            // Run until halt
            test_vm.run();

            totals.push(before.elapsed().as_nanos());

            // Verify that our register has reached MAX_ITERATIONS
            assert_eq!(test_vm.registers[1], test_vm.registers[2]);
            assert_eq!(test_vm.registers[4], 1134903170);
        }

        println!("Bytecode took: {:.2?}ns", (totals.iter().sum::<u128>() / totals.len() as u128));
    }
}