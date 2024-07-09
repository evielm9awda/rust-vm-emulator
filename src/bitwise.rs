pub struct BitwiseUnit;

impl BitwiseUnit {
    pub fn and(&self, a: i64, b: i64) -> i64 {
        a & b
    }

    pub fn or(&self, a: i64, b: i64) -> i64 {
        a | b
    }

    pub fn xor(&self, a: i64, b: i64) -> i64 {
        a ^ b
    }

    pub fn not(&self, a: i64) -> i64 {
        !a
    }

    pub fn shl(&self, a: i64, shift: u32) -> i64 {
        a << shift
    }

    pub fn shr(&self, a: i64, shift: u32) -> i64 {
        a >> shift
    }

    pub fn rol(&self, a: i64, shift: u32) -> i64 {
        a.rotate_left(shift)
    }

    pub fn ror(&self, a: i64, shift: u32) -> i64 {
        a.rotate_right(shift)
    }

    pub fn popcount(&self, a: i64) -> u32 {
        a.count_ones()
    }

    pub fn leading_zeros(&self, a: i64) -> u32 {
        a.leading_zeros()
    }

    pub fn trailing_zeros(&self, a: i64) -> u32 {
        a.trailing_zeros()
    }
}
