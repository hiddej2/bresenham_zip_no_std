use std::error::Error;
use bresenham_zip::bresenham3::Bresenham3ZipX;

fn main() -> Result<(), Box<dyn Error>> {
	println!("\nPairs along the X axis in 3D space: ");
	for (top, bottom) in Bresenham3ZipX::new((50, 50, 50), (0, 0, 0), (0, 100, 25))? {
		println!("{:?}, {:?}", top, bottom);
	}
	Ok(())
}
