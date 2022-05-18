mod mandelbrot;

use image::{ImageBuffer, RgbImage};
use num::complex::Complex32;

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
	let mut image: RgbImage = ImageBuffer::new(RES.0, RES.1);
	for x in 0..RES.0 {
		for y in 0..RES.1 {
			let pixel = mandelbrot_view.pixel(x, y);
			image.put_pixel(x, y, pixel);
		}
	}
	match image.save(FILENAME) {
		Ok(_) => println!("Saved image to {FILENAME}"),
		Err(err) => println!("Error saving image to {FILENAME}: {err}"),
	}
	let elapsed = start.elapsed().as_millis();
	println!("Run time: {elapsed} ms")
}
