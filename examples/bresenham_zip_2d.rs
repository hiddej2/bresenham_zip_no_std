use std::error::Error;
use bresenham_zip::bresenham2::{BresenhamZipX, BresenhamZipY};

fn main() -> Result<(), Box<dyn Error>> {
	println!("Pairs along the Y axis in 2D space: ");
	for (left, right) in BresenhamZipY::new((50, 50), (0, 100), (200, 100))? {
		println!("{:?}, {:?}", left, right);
	}

	println!("\nPairs along the X axis in 2D space: ");
	for (top, bottom) in BresenhamZipX::new((50, 50), (0, 0), (0, 100))? {
		println!("{:?}, {:?}", top, bottom);
	}
	Ok(())
}
