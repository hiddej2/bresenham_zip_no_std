use std::error::Error;
use bresenham_zip::{Axis, build_zip};
use bresenham_zip::zip_3d::Builder3d;

fn main() -> Result<(), Box<dyn Error>> {
	println!("\nPairs along the Z axis in 3D space: ");
	let zip = Builder3d::new()
		.axis(Axis::Z)
		.start_point((50, 50, 50))
		.first_ending_point((0, 0, 200))
		.second_ending_point((0, 100, 200))
		.build()?;
	for (a, b) in zip {
		println!("{:?}, {:?}", a, b);
	}

	println!("\nPairs along the X axis in 3D space: ");
	for (a, b) in build_zip!(3D:X - (50, 50, 50) -> (0, 0, 0), (0, 100, 200))? {
		println!("{:?}, {:?}", a, b);
	}

	Ok(())
}