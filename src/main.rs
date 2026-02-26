use clap::Parser;
use num_complex::Complex;

use fractale::arguments::{Args, FractalType};
use fractale::multi_core::multi_core_generate;
use fractale::traits::{Julia, Mandelbrot};

fn main() {
    let args = Args::parse();

    let img = match args.fractal_type {
        FractalType::Mandelbrot => multi_core_generate::<Mandelbrot>(Mandelbrot, &args),
        FractalType::Julia => {
            let k = Complex::new(args.julia_x, args.julia_y);
            multi_core_generate::<Julia>(Julia::from(k), &args)
        }
    };

    img.save(&args.output).expect("Saving failure");
}
