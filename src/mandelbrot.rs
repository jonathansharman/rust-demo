use image::Rgb;
use num::{complex::Complex32, Zero};

const COLORS: [Rgb<u8>; 13] = [
	Rgb([0; 3]),
	Rgb([0, 32, 64]),
	Rgb([0, 64, 128]),
	Rgb([0, 96, 192]),
	Rgb([0, 128, 255]),
	Rgb([0, 160, 255]),
	Rgb([0, 192, 255]),
	Rgb([0, 224, 255]),
	Rgb([0, 255, 255]),
	Rgb([64, 255, 255]),
	Rgb([128, 255, 255]),
	Rgb([192, 255, 255]),
	Rgb([255; 3]),
];

pub struct MandelbrotView {
	pub res: (u32, u32),
	pub max_iters: usize,
	pub c_min: Complex32,
	pub c_max: Complex32,
}

impl MandelbrotView {
	fn xy_to_c(&self, x: u32, y: u32) -> Complex32 {
		let (x, y) = (x as f32, y as f32);
		let (x_max, y_max) = ((self.res.0 - 1) as f32, (self.res.1 - 1) as f32);
		let re = self.c_min.re + x / x_max * (self.c_max.re - self.c_min.re);
		let im = self.c_max.im - y / y_max * (self.c_max.im - self.c_min.im);
		Complex32::new(re, im)
	}

	fn c_escape_time(&self, c: Complex32) -> Option<usize> {
		let mut z = Complex32::zero();
		for iters in 0..self.max_iters {
			z = z.powu(2) + c;
			if z.norm_sqr() >= 4.0 {
				return Some(iters);
			}
		}
		None
	}

	fn escape_time_pixel(&self, iters: Option<usize>) -> Rgb<u8> {
		match iters {
			Some(iters) => COLORS[iters as usize % COLORS.len()],
			None => Rgb([0; 3]),
		}
	}

	pub fn pixel(&self, x: u32, y: u32) -> Rgb<u8> {
		self.escape_time_pixel(self.c_escape_time(self.xy_to_c(x, y)))
	}
}
