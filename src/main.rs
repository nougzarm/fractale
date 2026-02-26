use image::{ImageBuffer, Rgb};
use num_complex::Complex;

fn mandelbrot(c: Complex<f64>, max_iter: usize) -> usize {
    let mut z = Complex::<f64>::new(0.0, 0.0);
    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn convert(width: u32, height: u32, (x_p, y_p): (u32, u32)) -> Result<Complex<f64>, ()> {
    if x_p > width || y_p > height {
        return Err(());
    }

    let x = 3.0 * (x_p as f64) / (width as f64) - 2.0;
    let y = 2.4 * (y_p as f64) / (height as f64) - 1.2;

    Ok(Complex::<f64>::new(x, y))
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of the output image (in pixels)
    #[arg(short = 'W', long, default_value_t = 800)]
    width: u32,

    /// Height of the output image (in pixels)
    #[arg(short = 'H', long, default_value_t = 800)]
    height: u32,

    /// Center of the view (X)
    #[arg(short = 'x', long, default_value_t = -0.75)]
    center_x: f64,

    /// Center of the view (Y)
    #[arg(short = 'y', long, default_value_t = 0.0)]
    center_y: f64,

    /// Zoom level
    #[arg(short, long, default_value_t = 1.0)]
    zoom: f64,

    /// Maximum number of iterations (precision)
    #[arg(short = 'i', long, default_value_t = 255)]
    max_iter: usize,

    /// Name of output file
    #[arg(short, long, default_value_t = String::from("mandelbrot.png"))]
    output: String,
}

fn main() {
    let args = Args::parse();

    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(args.width, args.height);

    for (x_p, y_p, pixel) in img.enumerate_pixels_mut() {
        let c = convert(args.width, args.height, (x_p, y_p)).unwrap();
        let i = mandelbrot(c, args.max_iter);

        if i == args.max_iter {
            *pixel = Rgb([0, 0, 0]);
        } else {
            let t = i as f64 / args.max_iter as f64;
            let r = (255.0 * t) as u8;
            let g = (255.0 * t * t) as u8;
            let b = (255.0 * t * t * t * t) as u8;
            *pixel = Rgb([r, g, b]);
        }
    }

    img.save("mandelbrot.png").expect("Failure");
}
