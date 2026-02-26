use crate::arguments::Args;
use crate::mandelbrot::Iteration;
use num_complex::Complex;

pub fn indice_determine<F: Iteration>(c: Complex<f64>, max_iter: usize) -> usize {
    let mut z = F::first();
    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = F::iter(z, c);
    }
    max_iter
}

pub fn convert(args: &Args, (x_p, y_p): (u32, u32)) -> Result<Complex<f64>, ()> {
    if x_p >= args.width || y_p >= args.height {
        return Err(());
    }

    let x_range = 3.0 / args.zoom;
    let y_range = 2.4 / args.zoom;

    let x = args.center_x + (x_p as f64 / args.width as f64 - 0.5) * x_range;
    let y = args.center_y + (y_p as f64 / args.height as f64 - 0.5) * y_range;

    Ok(Complex::<f64>::new(x, y))
}
