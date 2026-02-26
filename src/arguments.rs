use clap::Parser;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum FractalType {
    Mandelbrot,
    Julia,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Width of the output image (in pixels)
    #[arg(short = 'W', long, default_value_t = 800)]
    pub width: u32,

    /// Height of the output image (in pixels)
    #[arg(short = 'H', long, default_value_t = 800)]
    pub height: u32,

    /// Center of the view (X)
    #[arg(short = 'x', long, default_value_t = -0.75)]
    pub center_x: f64,

    /// Center of the view (Y)
    #[arg(short = 'y', long, default_value_t = 0.0)]
    pub center_y: f64,

    /// Zoom level
    #[arg(short = 'z', long, default_value_t = 1.0)]
    pub zoom: f64,

    /// Maximum number of iterations (precision)
    #[arg(short = 'i', long, default_value_t = 255)]
    pub max_iter: usize,

    #[arg(short = 't', long, value_enum, default_value_t = FractalType::Mandelbrot)]
    pub fractal_type: FractalType,

    /// Name of output file
    #[arg(short = 'o', long, default_value_t = String::from("output.png"))]
    pub output: String,
}
