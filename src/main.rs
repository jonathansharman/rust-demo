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
	let mut threads = Vec::new();
	let chunks = 8;
	for chunk in 0..chunks {
		threads.push(std::thread::spawn(|| {
			let y_min = chunk * RES.1 / chunks;
			let y_max = (chunk + 1) * RES.1 / chunks;
			for y in y_min..y_max {
				for x in 0..RES.0 {
					let pixel = mandelbrot_view.pixel(x, y);
					image.put_pixel(x, y, pixel);
				}
			}
		}));
	}
	for thread in threads {
		thread.join();
	}
	match image.save(FILENAME) {
		Ok(_) => println!("Saved image to {FILENAME}"),
		Err(err) => println!("Error saving image to {FILENAME}: {err}"),
	}
	let elapsed = start.elapsed().as_millis();
	println!("Run time: {elapsed} ms")
}
