use num_complex::Complex;

pub trait Iteration {
    fn first(&self, c: Complex<f64>) -> Complex<f64>;

    fn iter(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64>;
}

pub struct Mandelbrot;

impl Iteration for Mandelbrot {
    fn first(&self, _c: Complex<f64>) -> Complex<f64> {
        Complex::new(0.0, 0.0)
    }

    fn iter(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}

pub struct Julia(Complex<f64>);

impl Iteration for Julia {
    fn first(&self, c: Complex<f64>) -> Complex<f64> {
        c
    }

    fn iter(&self, z: Complex<f64>, _c: Complex<f64>) -> Complex<f64> {
        z * z + self.0
    }
}