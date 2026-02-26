use image::{ImageBuffer, Rgb};

use crate::arguments::Args;
use crate::mandelbrot::Mandelbrot;
use crate::utils::{coloring, convert, indice_determine};

pub fn single_core_generate(args: &Args) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(args.width, args.height);
    let mandelbrot = Mandelbrot;

    for (x_p, y_p, pixel) in img.enumerate_pixels_mut() {
        let c = convert(args, (x_p, y_p)).unwrap();
        let i = indice_determine::<Mandelbrot>(&mandelbrot, c, args.max_iter);

        *pixel = Rgb(coloring(i, args.max_iter as f64))
    }

    img
}
