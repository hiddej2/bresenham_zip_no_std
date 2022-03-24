use std::error::Error;
use bresenham_zip::{Axis, build_zip};
use bresenham_zip::zip::Builder;

fn main() -> Result<(), Box<dyn Error>> {

	println!("Pairs along the Y axis in 2D space: ");
	let zip = {
		Builder::new()
			.axis(Axis::X)
			.start_point((50, 50))
			.first_ending_point((0, 0))
			.second_ending_point((0, 100))
			.build()?
	};
	for (left, right) in zip {
		println!("{:?}, {:?}", left, right);
	}

	println!("\nPairs along the X axis in 2D space: ");
	for (top, bottom) in build_zip!(2:y - (50, 50) -> (0, 100), (100, 100))? {
		println!("{:?}, {:?}", top, bottom);
	}

	Ok(())
}
