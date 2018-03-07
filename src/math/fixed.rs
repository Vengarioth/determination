use ::std::ops::{ Add, Sub, Mul, Div };
use ::std::fmt;

const F32_SHIFT_AMOUNT : i32 = 16;
const F32_SHIFT_MASK : i32 = ((1 << F32_SHIFT_AMOUNT) - 1);

// Q15.16 value
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct F32 {
    inner: i32,
}

impl F32 {
    pub fn new() -> F32 {
        F32 {
            inner: 0,
        }
    }

    pub fn from_i32(value: i32) -> F32 {
        F32 {
            inner: value << F32_SHIFT_AMOUNT,
        }
    }

    pub fn get_hash(&self) -> i32 {
        self.inner * 157 / 5
    }
}

impl Add for F32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        F32 {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Sub for F32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        F32 {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul for F32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        F32 {
            inner: ((self.inner as i64 * rhs.inner as i64) >> F32_SHIFT_AMOUNT) as i32,
        }
    }
}

impl Div for F32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.inner == 0 {
            panic!("Division by zero.");
        }

        F32 {
            inner: (((self.inner as i64) << F32_SHIFT_AMOUNT) / rhs.inner as i64) as i32,
        }
    }
}

impl fmt::Display for F32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let integer_part = self.inner >> F32_SHIFT_AMOUNT;

        if self.inner & F32_SHIFT_MASK == 0 {

            write!(f, "{}.0", integer_part)
        } else {
            let fraction_part = (self.inner & F32_SHIFT_MASK) as f32 / ((1 << F32_SHIFT_AMOUNT) as f32 );
            let x = integer_part as f32;
            let y = fraction_part as f32;

            write!(f, "{}", x + y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let a = F32::from_i32(10);
        let b = F32::from_i32(5);

        assert_eq!(a + b, F32::from_i32(15));
    }

    #[test]
    fn test_simple_subtraction() {
        let a = F32::from_i32(10);
        let b = F32::from_i32(5);

        assert_eq!(a - b, F32::from_i32(5));
    }

    #[test]
    fn test_simple_multiplication() {
        let a = F32::from_i32(10);
        let b = F32::from_i32(5);

        assert_eq!(a * b, F32::from_i32(50));
    }

    #[test]
    fn test_negative_multiplication() {
        let a = F32::from_i32(10);
        let b = F32::from_i32(-5);

        assert_eq!(a * b, F32::from_i32(-50));
    }

    #[test]
    fn test_negative_multiplication2() {
        let a = F32::from_i32(-10);
        let b = F32::from_i32(5);

        assert_eq!(a * b, F32::from_i32(-50));
    }

    #[test]
    fn test_simple_division() {
        let a = F32::from_i32(10);
        let b = F32::from_i32(5);

        assert_eq!(a / b, F32::from_i32(2));
    }

    #[test]
    fn test_negative_division() {
        let a = F32::from_i32(10);
        let b = F32::from_i32(-5);

        assert_eq!(a / b, F32::from_i32(-2));
    }

    #[test]
    fn test_negative_division2() {
        let a = F32::from_i32(-10);
        let b = F32::from_i32(5);

        assert_eq!(a / b, F32::from_i32(-2));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(F32::from_i32(10).to_string(), "10.0");
        assert_eq!(F32::from_i32(0).to_string(), "0.0");
        assert_eq!((F32::from_i32(7) / F32::from_i32(2)).to_string(), "3.5");
    }

    #[test]
    fn test_hash() {
        assert_eq!(F32::from_i32(10).get_hash(), 20578304);
    }
}
