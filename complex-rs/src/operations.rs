use crate::complex::Complex;
use std::ops::{Add, Sub, Mul, Div};

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.r + other.r, self.i + other.i)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.r - other.r, self.i - other.i)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex::new(self.r * other.r - self.i * other.i, self.r * other.i + self.i * other.r)
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denom = other.r.powi(2) + other.i.powi(2);
        Complex::new((self.r * other.r + self.i * other.i) / denom, (self.i * other.r - self.r * other.i) / denom)
    }
}