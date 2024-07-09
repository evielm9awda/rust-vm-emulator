use crate::instruction::Instruction;

pub struct Memory {
    data: Vec<i64>,
    program: Vec<Instruction>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
            program: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    pub fn fetch(&self, address: usize) -> Instruction {
        self.program[address]
    }

    pub fn read(&self, address: usize) -> i64 {
        self.data[address]
    }

    pub fn write(&mut self, address: usize, value: i64) {
        self.data[address] = value;
    }
}
