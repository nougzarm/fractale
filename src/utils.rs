use crate::arguments::Args;
use crate::traits::Fractal;
use num_complex::Complex;

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

pub fn indice_determine<F: Fractal>(f: &F, c: Complex<f64>, max_iter: usize) -> f64 {
    let mut z = f.first(c);

    for i in 0..max_iter {
        if z.norm_sqr() > 256.0 {
            // Smooth Coloring
            let log_zn = z.norm().ln();
            let nu = log_zn.ln() / 2.0f64.ln();

            return i as f64 + 1.0 - nu;
        }
        z = f.iter(z, c);
    }

    max_iter as f64
}

pub fn coloring(iter_index: f64, max_iter: f64) -> [u8; 3] {
    if iter_index >= max_iter {
        [0, 0, 0]
    } else {
        use std::f64::consts::PI;

        let t = iter_index * 0.05;

        // Formula : a + b * cos(2π * (c * t + d))
        let r = (0.5 + 0.5 * (2.0 * PI * (1.0 * t + 0.00)).cos()) * 255.0;
        let g = (0.5 + 0.5 * (2.0 * PI * (1.0 * t + 0.15)).cos()) * 255.0;
        let b = (0.5 + 0.5 * (2.0 * PI * (1.0 * t + 0.20)).cos()) * 255.0;

        [r as u8, g as u8, b as u8]
    }
}
