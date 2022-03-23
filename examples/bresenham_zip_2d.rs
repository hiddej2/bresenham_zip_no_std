use std::error::Error;
use bresenham_zip::Axis;
use bresenham_zip::zip::Builder;

fn main() -> Result<(), Box<dyn Error>> {
	let mut builder = Builder::new();

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
	let zip = builder
		.axis(Axis::Y)
		.start_point((50, 50))
		.first_ending_point((0, 100))
		.second_ending_point((100, 100))
		.build()?;
	for (top, bottom) in zip {
		println!("{:?}, {:?}", top, bottom);
	}

	Ok(())
}
