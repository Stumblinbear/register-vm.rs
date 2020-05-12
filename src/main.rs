pub mod vm;
pub mod assembler;

use std::io;
use std::io::prelude::*;

use crate::vm::VM;
use crate::vm::instructions::Opcode;

const COMMAND_PREFIX: char = '.';

pub static BANNER: &str = "Rust Register-based Virtual Machine v1. Type .help for a list of codes; .quit to exit.";
pub static PROMPT: &str = ">>> ";

fn main() {
    println!("{}", BANNER);

    let mut vm = VM::default();

    let mut show_registers = true;

    'main: loop {
        if show_registers {
            println!("------------------------------------------------------------------------------------------");
            print!("Registers: ");

            for register in &vm.registers {
                print!("{:#04x} ", register);
            }

            println!();

            println!("Remainder: {}          Equality Flag: {}", vm.remainder, vm.equal_flag);
            
            println!("------------------------------------------------------------------------------------------");

            show_registers = false;
        }

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
            let args = &parts[1..];

            if command == "quit" {
                break 'main;
            } else if command == "registers" {
                show_registers = true;
            } else if command == "help" {
                if args.len() > 0 {
                    let op = Opcode::from(args[0].to_uppercase());
                    
                    println!("{}: {}", op.name(), op.info());
                } else {
                    println!("List of all Opcodes:");
                    for op in Opcode::all() {
                        println!("  {}: {}", op.instruction(), op.info());
                    }
                }
            } else {
                println!("Unknown command: {}", command);
            }
        } else {
            let tokens: Vec<&str> = input.split_whitespace().collect();
    
            let op = Opcode::from(tokens[0].to_uppercase());
    
            print!("{} ", op.instruction());
            
            let mut bytes: Vec<u8> = vec![ op.byte() ];
    
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

            vm.program.append(&mut bytes);
    
            println!();
    
            vm.run();
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
        const TIMES: u16 = u16::MAX;

        const MAX_ITERATIONS: u8 = 45;

        let mut target = 0;

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

            target = curr;
            
            totals.push(before.elapsed().as_nanos());
        }
        
        println!("Rust took: {:.2?}ns", (totals.iter().sum::<u128>() / totals.len() as u128));

        let mut totals: Vec<u128> = vec![];
        
        for _i in 0..TIMES {
            let mut test_vm = VM::default();

            test_vm.program = vec![
                Opcode::Set.byte(), 0, 1, 0,    // Set $0 to 1
                Opcode::JumpForward.byte(), 0,  // Jump forward $0 bytes
                Opcode::Halt.byte(),

                Opcode::Set.byte(), 0, 6, 0,   // Load $0 with 6: byte of the halt instruction

                Opcode::Set.byte(), 1, 0, 0,   // Reset $1 to 0
                Opcode::Set.byte(), 2, MAX_ITERATIONS - 1, 0,

                Opcode::Set.byte(), 3, 0, 0,   // Sets $3 to 0
                Opcode::Set.byte(), 4, 1, 0,   // Sets $4 to 1
                
                Opcode::Set.byte(), 6, 31, 0,  // Load $6 with 29: the start of the iteration

                // Start of fib iterations
                Opcode::Equal.byte(), 1, 2,     // Check if $1 (current) and $2 (max) are equal
                Opcode::JumpIfEqual.byte(), 0,  // Jump to byte $0 if equal flag is set

                Opcode::Increment.byte(), 1,    // Increment $1
                Opcode::Add.byte(), 5, 3, 4,    // $5 = $3 + $4
                Opcode::Move.byte(), 3, 4,      // $3 = $4
                Opcode::Move.byte(), 4, 5,      // $4 = $5
                
                Opcode::Jump.byte(), 6,         // Jump to byte $6
            ];

            let before = time::Instant::now();
            
            // Run until halt
            test_vm.run();

            totals.push(before.elapsed().as_nanos());

            // Verify that our register has reached MAX_ITERATIONS
            assert_eq!(test_vm.registers[1], test_vm.registers[2]);
            assert_eq!(test_vm.registers[4], target);
        }

        println!("Bytecode took: {:.2?}ns", (totals.iter().sum::<u128>() / totals.len() as u128));
    }
}