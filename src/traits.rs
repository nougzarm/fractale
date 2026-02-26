use num_complex::Complex;

pub trait Fractal: Default {
    fn first(&self, c: Complex<f64>) -> Complex<f64>;

    fn iter(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64>;
}

#[derive(Default)]
pub struct Mandelbrot;

impl Fractal for Mandelbrot {
    fn first(&self, _c: Complex<f64>) -> Complex<f64> {
        Complex::new(0.0, 0.0)
    }

    fn iter(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}

pub struct Julia(Complex<f64>);

impl Default for Julia {
    fn default() -> Self {
        Julia(Complex { re: -0.4, im: 0.6 })
    }
}

impl Fractal for Julia {
    fn first(&self, c: Complex<f64>) -> Complex<f64> {
        c
    }

    fn iter(&self, z: Complex<f64>, _c: Complex<f64>) -> Complex<f64> {
        z * z + self.0
    }
}
