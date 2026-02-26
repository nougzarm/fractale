use num_complex::Complex;

pub trait Iteration {
    fn first() -> Complex<f64>;

    fn iter(z: Complex<f64>, c: Complex<f64>) -> Complex<f64>;
}

pub struct Mandelbrot;

impl Iteration for Mandelbrot {
    fn first() -> Complex<f64> {
        Complex::new(0.0, 0.0)
    }

    fn iter(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}
