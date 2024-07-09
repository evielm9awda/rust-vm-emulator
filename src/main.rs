mod vm;
mod instruction;
mod memory;
mod register;
mod stack;
mod arithmetic;
mod bitwise;
mod control_flow;
mod function;

use vm::VirtualMachine;
use instruction::Instruction;

fn main() {
    let mut vm = VirtualMachine::new(1024);
    
    // Example program
    let program = vec![
        Instruction::LoadImmediate(0, 10),
        Instruction::LoadImmediate(1, 20),
        Instruction::Add(2, 0, 1),
        Instruction::Print(2),
        Instruction::Halt,
    ];

    vm.load_program(program);
    vm.run();
}
