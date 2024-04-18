//! Contains the logic to build new two-dimensional BresenhamZips

use crate::{Axis, Point2, SignedNum};
use crate::util::Point;
use crate::zip::BresenhamZip;

const MAX_ACCEPTED_AXIS: u8 = 1;

/// Builder to construct a new [BresenhamZip]. It is required to specify the starting point and two
/// ending points, both of them **must share the same value in the axis** of the zip to build.
/// This axis is also required in the building pipeline.
///
/// ```
/// # use std::error::Error;
/// # pub fn main() -> Result<(), Box<dyn Error>> {
/// let zip = bresenham_zip::zip::Builder::new()
///   .axis(bresenham_zip::Axis::Y)
///   .start_point((50, 50))
///   .first_ending_point((0, 100))
///   .second_ending_point((100, 100))
///   .build()?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Builder<T> {
	start: Option<Point2<T>>,
	end_a: Option<Point2<T>>,
	end_b: Option<Point2<T>>,
	axis: u8
}

impl<T: SignedNum> Builder<T> {

	/// Creates a new Builder ready to be configured
	pub fn new() -> Builder<T> {
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
	pub fn axis(&mut self, axis: Axis) -> &mut Builder<T> {
		match axis {
			Axis::X => self.axis = 0,
			Axis::Y => self.axis = 1,
			_ => self.axis = MAX_ACCEPTED_AXIS + 1,
		};
		self
	}

	/// Specifies the starting point for both the lines to be drawn in the BresenhamZip
	///
	/// * `start` - Starting point of type (T, T)
	///
	pub fn start_point(&mut self, start: Point2<T>) -> &mut Builder<T> {
		self.start = Some(start);
		self
	}

	/// Specifies the ending point for one of the lines to be drawn in the BresenhamZip
	///
	/// * `end` - Ending point of one line, must be of type (T, T)
	///
	pub fn first_ending_point(&mut self, end: Point2<T>) -> &mut Builder<T> {
		self.end_a = Some(end);
		self
	}

	/// Specifies the ending point for one of the lines to be drawn in the BresenhamZip
	///
	/// * `end` - Ending point of one line, must be of type (T, T)
	///
	pub fn second_ending_point(&mut self, end: Point2<T>) -> &mut Builder<T> {
		self.end_b = Some(end);
		self
	}

	/// Builds the BresenhamZip corresponding with the defined arguments
	///
	/// # Error
	/// This call can generate the following errors
	///
	/// * [Error::MissingAxis], if no axis was specified.
	/// * [Error::MissingPoint], if any of the three points is missing.
	/// * [Error::InvalidX], if the axis is X and the two ending points have divergent X values.
	/// * [Error::InvalidY], if the axis is Y and the two ending points have divergent Y values.
	///
	pub fn build<'a, 'b>(&'b self) -> BresenhamZip<T> {
		if self.axis > MAX_ACCEPTED_AXIS {
			
		}
		let axis = self.axis;

		match (&self.start, &self.end_a, &self.end_b) {
			(Some(start), Some(end_a), Some(end_b)) => {
				if !(end_a.nth(axis) != end_b.nth(axis)) {
					return BresenhamZip::new(*start, *end_a, *end_b, self.axis);
				}
				else{
					return BresenhamZip::new(*start, *end_a, *end_b, self.axis);
				}
			},
			_ => panic!(),
		}
	}

}

#[cfg(test)]
mod test {
	use crate::{Axis, build_zip};
	use core::error::Error;
	use crate::zip::Builder;

	#[test]
	fn invalid_axis() {

	}

	#[test]
	fn missing_point() {

	}

	#[test]
	fn invalid_points() {

	}

	#[test]
	fn valid() {

	}

}