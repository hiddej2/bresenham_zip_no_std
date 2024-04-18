//! Contains the logic to build new two-dimensional BresenhamZips

use crate::{Axis, Point3, SignedNum};
use crate::util::Point;
use crate::zip_3d::Bresenham3dZip;

const MAX_ACCEPTED_AXIS: u8 = 2;


/// Builder to construct a new [Bresenham3dZip]. It is required to specify the starting point and two
/// ending points, both of them **must share the same value in the axis** of the zip to build.
/// This axis is also required in the building pipeline.
///
/// ```
/// # use std::error::Error;
/// # pub fn main() -> Result<(), Box<dyn Error>> {
/// let zip = bresenham_zip::zip_3d::Builder3d::new()
///   .axis(bresenham_zip::Axis::Z)
///   .start_point((50, 50, 50))
///   .first_ending_point((0, 100, 200))
///   .second_ending_point((100, 100, 200))
///   .build()?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Builder3d<T> {
	start: Option<Point3<T>>,
	end_a: Option<Point3<T>>,
	end_b: Option<Point3<T>>,
	axis: u8
}

impl<T: SignedNum> Builder3d<T> {

	/// Creates a new Builder3d ready to be configured
	pub fn new() -> Builder3d<T> {
		Self {
			start: None,
			end_a: None,
			end_b: None,
			axis: MAX_ACCEPTED_AXIS + 1
		}
	}

	/// Specifies the axis that will be used to generate the points of the lines during the iteration.
	/// The returned tuples will share the same value in the specified axis.
	///
	/// * `axis` - Axis to use in the Zip iteration
	///
	pub fn axis(&mut self, axis: Axis) -> &mut Builder3d<T> {
		match axis {
			Axis::X => self.axis = 0,
			Axis::Y => self.axis = 1,
			Axis::Z => self.axis = 2,
		};
		self
	}

	/// Specifies the starting point for both the lines to be drawn in the [Bresenham3dZip]
	///
	/// * `start` - Starting point of type (T, T, T)
	///
	pub fn start_point(&mut self, start: Point3<T>) -> &mut Builder3d<T> {
		self.start = Some(start);
		self
	}

	/// Specifies the ending point for one of the lines to be drawn in the [Bresenham3dZip]
	///
	/// * `end` - Ending point of one line, must be of type (T, T, T)
	///
	pub fn first_ending_point(&mut self, end: Point3<T>) -> &mut Builder3d<T> {
		self.end_a = Some(end);
		self
	}

	/// Specifies the ending point for one of the lines to be drawn in the [Bresenham3dZip]
	///
	/// * `end` - Ending point of one line, must be of type (T, T, T)
	///
	pub fn second_ending_point(&mut self, end: Point3<T>) -> &mut Builder3d<T> {
		self.end_b = Some(end);
		self
	}

	/// Builds the [Bresenham3dZip] corresponding with the defined arguments
	///
	/// # Error
	/// This call can generate the following errors
	///
	/// * [Error::MissingAxis], if no axis was specified.
	/// * [Error::MissingPoint], if any of the three points is missing.
	/// * [Error::InvalidX], if the axis is X and the two ending points have divergent X values.
	/// * [Error::InvalidY], if the axis is Y and the two ending points have divergent Y values.
	/// * [Error::InvalidZ], if the axis is Z and the two ending points have divergent Y values.
	///
	pub fn build<'a, 'b>(&'b self) {
		if self.axis > MAX_ACCEPTED_AXIS {
			return;
		}
		let axis = self.axis;

		match (&self.start, &self.end_a, &self.end_b) {
			(None, _, _) => (),
			(_, None, _) => (),
			(_, _, None) => (),
			(Some(start), Some(end_a), Some(end_b)) => {
				if end_a.nth(axis) != end_b.nth(axis) {
					();
				} else {
					Bresenham3dZip::new(*start, *end_a, *end_b, self.axis);
				}
			}
		}
	}

}

#[cfg(test)]
mod test {
	use crate::{Axis, build_zip};
	use core::error::Error;
	use crate::zip_3d::Builder3d;

	#[test]
	fn missing_axis() {

	}

	#[test]
	fn missing_point() {

	}

	#[test]
	fn invalid_points() {

	}

	#[test]
	fn valid() {
		// // Direct building
		// assert_eq!(format!("{:?}", build_zip!(3D:X - (50, 50, 50) -> (0, 0, 0), (0, 100, 200)).unwrap()),
		//            "Bresenham3dZip [ (50, 50, 50), (50, 50, 50) ]. Goal: 0");
		// // Modified building
		// let built = Builder3d::new()
		// 	.axis(Axis::X)
		// 	.axis(Axis::Y)
		// 	.start_point((25, 25, 25))
		// 	.second_ending_point((50, 50, 50))
		// 	.start_point((10, 10, 10))
		// 	.first_ending_point((0, 100, 0))
		// 	.second_ending_point((100, 100, 100))
		// 	.build();
		// assert_eq!(format!("{:?}", built.unwrap()), "Bresenham3dZip [ (10, 10, 10), (10, 10, 10) ]. Goal: 100");
	}

}