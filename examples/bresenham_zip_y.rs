use std::error::Error;
use bresenham_zip::bresenham::BresenhamZipY;

fn main() -> Result<(), Box<dyn Error>> {
	for (left, right) in BresenhamZipY::new((50, 50), (0, 100), (200, 100))? {
		println!("{:?}, {:?}", left, right);
	}
	Ok(())
}
