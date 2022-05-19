mod mandelbrot;

use image::{ImageBuffer, RgbImage};
use num::complex::Complex32;
use rayon::prelude::*;

const RES: (u32, u32) = (5000, 5000);
const FILENAME: &str = "out/m-set.png";

fn main() {
	let start = std::time::Instant::now();
	let mandelbrot_view = mandelbrot::MandelbrotView {
		res: RES,
		max_iters: 500,
		c_min: Complex32::new(-2.5, -1.75),
		c_max: Complex32::new(1.0, 1.75),
	};

	let mut buf = vec![[0; 3]; (RES.0 * RES.1) as usize];
	buf.par_iter_mut().enumerate().for_each(|(i, pixel)| {
		let x = i % RES.0 as usize;
		let y = i / RES.0 as usize;
		*pixel = mandelbrot_view.pixel(x as u32, y as u32).0;
	});
	let buf = buf.concat();
	let image: RgbImage = ImageBuffer::from_vec(RES.0, RES.1, buf).unwrap();

	let elapsed_generate = start.elapsed().as_millis();
	println!("M-set generation time: {elapsed_generate} ms");

	match image.save(FILENAME) {
		Ok(_) => println!("Saved image to {FILENAME}"),
		Err(err) => println!("Error saving image to {FILENAME}: {err}"),
	}

	let elapsed_total = start.elapsed().as_millis();
	println!("Total run time: {elapsed_total} ms")
}
