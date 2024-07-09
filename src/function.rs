use crate::stack::Stack;

pub struct FunctionUnit {
    call_stack: Vec<usize>,
}

impl FunctionUnit {
    pub fn new() -> Self {
        FunctionUnit {
            call_stack: Vec::new(),
        }
    }

    pub fn call(&mut self, stack: &mut Stack, return_address: usize) {
        stack.push(return_address as i64);
        self.call_stack.push(return_address);
    }

    pub fn ret(&mut self, stack: &mut Stack) -> Option<usize> {
        if let Some(return_address) = stack.pop() {
            self.call_stack.pop();
            Some(return_address as usize)
        } else {
            None
        }
    }

    pub fn call_depth(&self) -> usize {
        self.call_stack.len()
    }

    pub fn current_frame(&self) -> Option<usize> {
        self.call_stack.last().cloned()
    }

    pub fn clear_call_stack(&mut self) {
        self.call_stack.clear();
    }
}
