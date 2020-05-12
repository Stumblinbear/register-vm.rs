use crate::assembler::header::Header;

#[derive(Default)]
pub struct Program {
    pub header: Header,

    pub read_only: Vec<u8>,
    pub bytecode: Vec<u8>,
}

impl Program {
    pub fn load() {
        // TODO: Load from stream
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_program() {
        Program::default();
    }
}