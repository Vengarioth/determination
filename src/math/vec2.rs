use ::std::ops::{ Add, Sub, Mul, Div };
use ::std::fmt;
use ::math::fixed::F32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vec2 {
    x: F32,
    y: F32,
}

impl Vec2 {
    pub fn new(x: F32, y: F32) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn magnitude_squared(&self) -> F32 {
        ((self.x * self.x) + (self.y * self.y))
    }

    pub fn get_hash(&self) -> i32 {
        let mut hash = 0;
        hash = hash * 37 + self.x.get_hash() / 5;
        hash = hash * 131 + self.y.get_hash() / 5;
        hash
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vec2({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let value = Vec2::new(F32::from_i32(15), F32::from_i32(11));
        assert_eq!(value.to_string(), "vec2(15.0, 11.0)");
    }
    
    #[test]
    fn test_hash() {
        assert_eq!(Vec2::new(F32::from_i32(15), F32::from_i32(11)).get_hash(), 813254547);
    }
}
