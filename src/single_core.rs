use image::{ImageBuffer, Rgb};

use crate::arguments::Args;
use crate::utils::{convert, mandelbrot};

pub fn single_core_generate(args: &Args) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(args.width, args.height);

    for (x_p, y_p, pixel) in img.enumerate_pixels_mut() {
        let c = convert(args, (x_p, y_p)).unwrap();
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

    img
}
