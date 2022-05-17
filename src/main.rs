use image::{ImageBuffer, Rgb, RgbImage};

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const FILENAME: &str = "out/stripes.png";

fn main() {
	let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
	for x in 0..WIDTH {
		for y in 0..HEIGHT {
			let pixel = if (x + y) / 10 % 2 == 0 {
				Rgb([0, 0, 0])
			} else {
				Rgb([255, 255, 255])
			};
			image.put_pixel(x, y, pixel);
		}
	}
	match image.save(FILENAME) {
		Ok(_) => println!("Saved image to {FILENAME}"),
		Err(err) => println!("Error saving image to {FILENAME}: {err}"),
	}
}
