use crate::arguments::Args;
use crate::mandelbrot::Iteration;
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

pub fn indice_determine<F: Iteration>(c: Complex<f64>, max_iter: usize) -> f64 {
    let mut z = F::first();

    for i in 0..max_iter {
        if z.norm_sqr() > 256.0 {
            // Smooth Coloring
            let log_zn = z.norm().ln();
            let nu = log_zn.ln() / 2.0f64.ln();

            return i as f64 + 1.0 - nu;
        }
        z = F::iter(z, c);
    }

    max_iter as f64
}

pub fn coloring(iter_index: f64, max_iter: f64) -> [u8; 3] {
    if iter_index >= max_iter {
        return [0, 0, 0];
    } else {
        let t = iter_index.sqrt() * 0.5;
        let r = (t.sin() * 127.0 + 128.0) as u8;
        let g = ((t + 2.0).sin() * 127.0 + 128.0) as u8;
        let b = ((t + 4.0).sin() * 127.0 + 128.0) as u8;

        return [r, g, b];
    }
}
