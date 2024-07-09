pub struct Stack {
    data: Vec<i64>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: i64) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&i64> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
