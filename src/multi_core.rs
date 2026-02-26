use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

use crate::arguments::Args;
use crate::mandelbrot::Mandelbrot;
use crate::utils::{convert, indice_determine};

pub fn multi_core_generate(args: &Args) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut raw_pixels = vec![0u8; (args.width * args.height * 3) as usize];

    raw_pixels
        .par_chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, pixel_slice)| {
            let x_p = (i as u32) % args.width;
            let y_p = (i as u32) / args.width;

            let c = convert(args, (x_p, y_p)).unwrap();
            let iterations = indice_determine::<Mandelbrot>(c, args.max_iter);

            if iterations == args.max_iter {
                pixel_slice[0] = 0;
                pixel_slice[1] = 0;
                pixel_slice[2] = 0;
            } else {
                let t = iterations as f64 / args.max_iter as f64;
                pixel_slice[0] = (255.0 * t) as u8;
                pixel_slice[1] = (255.0 * t * t) as u8;
                pixel_slice[2] = (255.0 * t * t * t * t) as u8;
            }
        });

    ImageBuffer::from_raw(args.width, args.height, raw_pixels).expect("Error")
}
