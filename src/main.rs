use clap::Parser;

use fractale::arguments::{Args, FractalType};
use fractale::multi_core::multi_core_generate;
use fractale::traits::{Julia, Mandelbrot};

fn main() {
    let args = Args::parse();

    let img = match args.fractal_type {
        FractalType::Mandelbrot => multi_core_generate::<Mandelbrot>(Mandelbrot, &args),
        FractalType::Julia => multi_core_generate::<Julia>(Julia::default(), &args),
    };

    img.save(&args.output).expect("Saving failure");
}
