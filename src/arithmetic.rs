pub struct ArithmeticUnit;

impl ArithmeticUnit {
    pub fn add(&self, a: i64, b: i64) -> i64 {
        a.wrapping_add(b)
    }

    pub fn sub(&self, a: i64, b: i64) -> i64 {
        a.wrapping_sub(b)
    }

    pub fn mul(&self, a: i64, b: i64) -> i64 {
        a.wrapping_mul(b)
    }

    pub fn div(&self, a: i64, b: i64) -> i64 {
        if b == 0 {
            panic!("Division by zero");
        }
        a.wrapping_div(b)
    }

    pub fn rem(&self, a: i64, b: i64) -> i64 {
        if b == 0 {
            panic!("Division by zero");
        }
        a.wrapping_rem(b)
    }

    pub fn pow(&self, base: i64, exponent: u32) -> i64 {
        base.wrapping_pow(exponent)
    }

    pub fn sqrt(&self, x: i64) -> i64 {
        if x < 0 {
            panic!("Cannot calculate square root of negative number");
        }
        (x as f64).sqrt() as i64
    }

    pub fn abs(&self, x: i64) -> i64 {
        x.abs()
    }

    pub fn sign(&self, x: i64) -> i64 {
        x.signum()
    }
}
