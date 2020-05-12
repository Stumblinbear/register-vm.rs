pub mod instructions;

mod test;

use crate::vm::instructions::Opcode;

#[derive(Default)]
pub struct VM {
    // The program counter keeps track of how many instructions have been executed
    ic: usize,
    
    // The program counter keeps track of which byte is being executed
    pub pc: usize,
    
    // The bytecode of our program
    pub program: Vec<u8>,
    
    /// Array that simulates hardware registers
    pub registers: [i32; 16],

    // Contains the remainder of division operations
    pub remainder: u32,

    pub float_registers: [f64; 32],

    /// Contains the result of the last comparison operation
    pub equal_flag: bool,

    pub heap: Vec<u8>,
}

impl VM {
    pub fn next_8_bits(&mut self) -> u8 {
        self.pc += 1;
        self.program[self.pc - 1]
    }
    
    pub fn next_16_bits(&mut self) -> u16 {
        self.pc += 2;
        (u16::from(self.program[self.pc - 2]) << 8) | u16::from(self.program[self.pc - 1])
    }

    // Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        loop {
            if !self.execute_instruction() {
                break;
            }
        }
    }

    // Executes one instruction. Meant to allow for more controlled execution of the VM.
    pub fn run_once(&mut self) -> bool {
        self.execute_instruction()
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        self.ic += 1;

        let opcode = self.next_8_bits();

        Opcode::call(self, opcode)
    }
}