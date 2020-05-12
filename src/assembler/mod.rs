pub mod header;
pub mod program;

#[derive(Default)]
pub struct Assembler {
    pub result: program::Program
}

impl Assembler {
    pub fn compile(&mut self) {

    }
}

pub struct AssemblerPhase {
    
}