mod complex;
mod operations;

use complex::Complex;

fn main() {
    let z1 = Complex::new(1.0, 2.0);
    let z2 = Complex::new(3.0, 4.0);

    let sum = z1.clone() + z2.clone();
    let product = z1.clone() * z2.clone();
    let quotient = z1.clone() / z2.clone();
    let power = z1.pow(2.0);
    let exponential = z1.exp();
    let square_root = z1.sqrt();
    let sine = z1.sin();
    let cosine = z1.cos();
    let tangent = z1.tan();
    let hyperbolic_sine = z1.sinh();
    let hyperbolic_cosine = z1.cosh();
    let hyperbolic_tangent = z1.tanh();

    println!("Sum: {:?}", sum);
    println!("Product: {:?}", product);
    println!("Quotient: {:?}", quotient);
    println!("Power: {:?}", power);
    println!("Exponential: {:?}", exponential);
    println!("Square Root: {:?}", square_root);
    println!("Sine: {:?}", sine);
    println!("Cosine: {:?}", cosine);
    println!("Tangent: {:?}", tangent);
    println!("Hyperbolic Sine: {:?}", hyperbolic_sine);
    println!("Hyperbolic Cosine: {:?}", hyperbolic_cosine);
    println!("Hyperbolic Tangent: {:?}", hyperbolic_tangent);
}