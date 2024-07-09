pub struct Registers {
    data: [i64; 32],
}

impl Registers {
    pub fn new() -> Self {
        Registers { data: [0; 32] }
    }

    pub fn get(&self, index: usize) -> i64 {
        self.data[index]
    }

    pub fn set(&mut self, index: usize, value: i64) {
        self.data[index] = value;
    }
}
