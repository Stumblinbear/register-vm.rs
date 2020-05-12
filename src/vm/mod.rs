pub mod instructions;
mod test;

use std::convert::TryInto;

use crate::vm::instructions::Opcode;

#[derive(Default)]
pub struct VM {
    // The program counter keeps track of how many instructions have been executed
    pub ic: usize,
    
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
    pub fn read_u8(&mut self) -> u8 {
        self.pc += 1;
        self.program[self.pc - 1]
    }
    
    pub fn read_u16(&mut self) -> u16 {
        self.pc += 2;
        u16::from_ne_bytes(self.program[(self.pc - 2)..(self.pc)].try_into().expect("Mismatched byte count."))
    }

    
    pub fn fetch_heap_u8(&mut self, pointer: usize) -> u8 {
        self.heap[pointer]
    }
    
    pub fn set_heap_u8(&mut self, pointer: usize, value: u8) {
        self.heap[pointer] = value;
    }

    
    pub fn fetch_heap_u16(&mut self, pointer: usize) -> u16 {
        u16::from_ne_bytes(self.heap[pointer..pointer + 2].try_into().expect("Mismatched byte count."))
    }
    
    pub fn set_heap_u16(&mut self, pointer: usize, value: u16) {
        let u8s: [u8; 2] = unsafe { std::mem::transmute(value.to_le()) };

        self.heap[pointer] = u8s[0];
        self.heap[pointer + 1] = u8s[1];
    }

    
    pub fn fetch_heap_u32(&mut self, pointer: usize) -> u32 {
        u32::from_ne_bytes(self.heap[pointer..pointer + 4].try_into().expect("Mismatched byte count."))
    }
    
    pub fn set_heap_u32(&mut self, pointer: usize, value: u32) {
        let u8s: [u8; 4] = unsafe { std::mem::transmute(value.to_le()) };

        self.heap[pointer] = u8s[0];
        self.heap[pointer + 1] = u8s[1];
        self.heap[pointer + 2] = u8s[2];
        self.heap[pointer + 3] = u8s[3];
    }

    
    pub fn fetch_heap_u64(&mut self, pointer: usize) -> u64 {
        u64::from_ne_bytes(self.heap[pointer..pointer + 8].try_into().expect("Mismatched byte count."))
    }
    
    pub fn set_heap_u64(&mut self, pointer: usize, value: u64) {
        let u8s: [u8; 8] = unsafe { std::mem::transmute(value.to_le()) };

        self.heap[pointer] = u8s[0];
        self.heap[pointer + 1] = u8s[1];
        self.heap[pointer + 2] = u8s[2];
        self.heap[pointer + 3] = u8s[3];
        self.heap[pointer + 4] = u8s[4];
        self.heap[pointer + 5] = u8s[5];
        self.heap[pointer + 6] = u8s[6];
        self.heap[pointer + 7] = u8s[7];
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

        let opcode = self.read_u8();

        Opcode::call(self, opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        VM::default()
    }

    #[test]
    fn set_u8_heap() {
        let mut test_vm = get_test_vm();

        test_vm.heap = vec![0, 0, 0, 0, 0, 69];
        test_vm.set_heap_u8(1, 8);
        
        assert_eq!(test_vm.heap, vec![0, 8, 0, 0, 0, 69]);
        
        assert_eq!(test_vm.fetch_heap_u32(1), 8);
    }

    #[test]
    fn set_u16_heap() {
        let mut test_vm = get_test_vm();

        test_vm.heap = vec![0, 0, 0, 0, 0, 69];
        test_vm.set_heap_u16(1, 257);
        
        assert_eq!(test_vm.heap, vec![0, 1, 1, 0, 0, 69]);
        
        assert_eq!(test_vm.fetch_heap_u32(1), 257);
    }

    #[test]
    fn set_u32_heap() {
        let mut test_vm = get_test_vm();

        test_vm.heap = vec![0, 0, 0, 0, 0, 69];
        test_vm.set_heap_u32(1, 500);
        
        assert_eq!(test_vm.heap, vec![0, 244, 1, 0, 0, 69]);
        
        assert_eq!(test_vm.fetch_heap_u32(1), 500);
    }

    #[test]
    fn set_u64_heap() {
        let mut test_vm = get_test_vm();

        test_vm.heap = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 69];
        test_vm.set_heap_u64(1, 2162110581051415215);
        
        assert_eq!(test_vm.heap, vec![0, 175, 218, 174, 60, 30, 92, 1, 30, 69]);
        
        assert_eq!(test_vm.fetch_heap_u64(1), 2162110581051415215u64);
    }
}