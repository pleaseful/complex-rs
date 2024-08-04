use std::ops::Div;

#[derive(Debug, Clone)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Self {
        Complex { r, i }
    }

    pub fn abs(&self) -> f64 {
        (self.r.powi(2) + self.i.powi(2)).sqrt()
    }

    pub fn conj(&self) -> Complex {
        Complex::new(self.r, -self.i)
    }

    pub fn arg(&self) -> f64 {
        self.i.atan2(self.r)
    }

    pub fn pow(&self, exponent: f64) -> Complex {
        let r = self.abs().powf(exponent);
        let theta = self.arg() * exponent;
        Complex::new(r * theta.cos(), r * theta.sin())
    }

    pub fn exp(&self) -> Complex {
        let exp_r = self.r.exp();
        Complex::new(exp_r * self.i.cos(), exp_r * self.i.sin())
    }

    pub fn sqrt(&self) -> Complex {
        let r = (self.abs()).sqrt();
        let theta = self.arg() / 2.0;
        Complex::new(r * theta.cos(), r * theta.sin())
    }

    pub fn sin(&self) -> Complex {
        Complex::new(self.i.sin(), self.i.cos())
    }

    pub fn cos(&self) -> Complex {
        Complex::new(self.i.cos(), -self.i.sin())
    }

    pub fn tan(&self) -> Complex {
        self.sin().div(self.cos())
    }


    pub fn sinh(&self) -> Complex {
        // Using formula sinh(z) = (e^z - e^(-z)) / 2
        let e_pos = self.exp();
        let e_neg = self.conj().exp(); // e^(-z) can be gotten by taking the exp of the conjugate
        Complex::new((e_pos.r - e_neg.r) / 2.0, (e_pos.i - e_neg.i) / 2.0)
    }

    pub fn cosh(&self) -> Complex {
        // Using formula cosh(z) = (e^z + e^(-z)) / 2
        let e_pos = self.exp();
        let e_neg = self.conj().exp();
        Complex::new((e_pos.r + e_neg.r) / 2.0, (e_pos.i + e_neg.i) / 2.0)
    }

    pub fn reciprocal(&self) -> Complex {
        let denominator = self.r.powi(2) + self.i.powi(2);
        Complex::new(self.r / denominator, -self.i / denominator)
    }



    pub fn tanh(&self) -> Complex {
        self.sinh().div(self.cosh())
    }

    pub fn division_alternative(&self, other: &Complex) -> Complex {
        let reciprocal = other.reciprocal();
self.clone() * reciprocal
    }


    pub fn log(&self) -> Complex {
        let r = self.abs().ln();
        let theta = self.arg();
        Complex::new(r, theta)
    }

}