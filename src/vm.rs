use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::register::Registers;
use crate::stack::Stack;
use crate::arithmetic::ArithmeticUnit;
use crate::bitwise::BitwiseUnit;
use crate::control_flow::ControlFlowUnit;
use crate::function::FunctionUnit;

pub struct VirtualMachine {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    program_counter: usize,
    arithmetic_unit: ArithmeticUnit,
    bitwise_unit: BitwiseUnit,
    control_flow_unit: ControlFlowUnit,
    function_unit: FunctionUnit,
    halted: bool,
}

impl VirtualMachine {
    pub fn new(memory_size: usize) -> Self {
        VirtualMachine {
            memory: Memory::new(memory_size),
            registers: Registers::new(),
            stack: Stack::new(),
            program_counter: 0,
            arithmetic_unit: ArithmeticUnit,
            bitwise_unit: BitwiseUnit,
            control_flow_unit: ControlFlowUnit,
            function_unit: FunctionUnit::new(),
            halted: false,
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.memory.load_program(program);
    }

    pub fn run(&mut self) {
        while !self.halted {
            let instruction = self.memory.fetch(self.program_counter);
            self.execute(instruction);
            self.program_counter += 1;
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LoadImmediate(reg, value) => {
                self.registers.set(reg, value);
            }
            Instruction::Add(dest, src1, src2) => {
                let result = self.arithmetic_unit.add(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::Sub(dest, src1, src2) => {
                let result = self.arithmetic_unit.sub(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::Mul(dest, src1, src2) => {
                let result = self.arithmetic_unit.mul(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::Div(dest, src1, src2) => {
                let result = self.arithmetic_unit.div(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::And(dest, src1, src2) => {
                let result = self.bitwise_unit.and(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::Or(dest, src1, src2) => {
                let result = self.bitwise_unit.or(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::Xor(dest, src1, src2) => {
                let result = self.bitwise_unit.xor(
                    self.registers.get(src1),
                    self.registers.get(src2),
                );
                self.registers.set(dest, result);
            }
            Instruction::Not(dest, src) => {
                let result = self.bitwise_unit.not(self.registers.get(src));
                self.registers.set(dest, result);
            }
            Instruction::Jump(address) => {
                self.program_counter = address;
            }
            Instruction::JumpIfZero(reg, address) => {
                if self.registers.get(reg) == 0 {
                    self.program_counter = address;
                }
            }
            Instruction::JumpIfNotZero(reg, address) => {
                if self.registers.get(reg) != 0 {
                    self.program_counter = address;
                }
            }
            Instruction::Call(address) => {
                self.function_unit.call(&mut self.stack, self.program_counter);
                self.program_counter = address;
            }
            Instruction::Return => {
                if let Some(return_address) = self.function_unit.ret(&mut self.stack) {
                    self.program_counter = return_address;
                }
            }
            Instruction::Push(reg) => {
                self.stack.push(self.registers.get(reg));
            }
            Instruction::Pop(reg) => {
                if let Some(value) = self.stack.pop() {
                    self.registers.set(reg, value);
                }
            }
            Instruction::Print(reg) => {
                println!("Register {}: {}", reg, self.registers.get(reg));
            }
            Instruction::Halt => {
                self.halted = true;
            }
        }
    }
}
