pub struct ControlFlowUnit;

impl ControlFlowUnit {
    pub fn compare(&self, a: i64, b: i64) -> i64 {
        if a < b {
            -1
        } else if a > b {
            1
        } else {
            0
        }
    }

    pub fn is_zero(&self, x: i64) -> bool {
        x == 0
    }

    pub fn is_positive(&self, x: i64) -> bool {
        x > 0
    }

    pub fn is_negative(&self, x: i64) -> bool {
        x < 0
    }

    pub fn is_even(&self, x: i64) -> bool {
        x % 2 == 0
    }

    pub fn is_odd(&self, x: i64) -> bool {
        x % 2 != 0
    }

    pub fn max(&self, a: i64, b: i64) -> i64 {
        std::cmp::max(a, b)
    }

    pub fn min(&self, a: i64, b: i64) -> i64 {
        std::cmp::min(a, b)
    }

    pub fn clamp(&self, x: i64, min: i64, max: i64) -> i64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
}
