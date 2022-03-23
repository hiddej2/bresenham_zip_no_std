//! Contains the logic to build new two-dimensional BresenhamZips

use crate::{Axis, nth, Point2, SignedNum};
use crate::error::Error;
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
	pub fn build<'a, 'b>(&'b self) -> Result<BresenhamZip<T>, Error<'a, T>> {
		if self.axis > MAX_ACCEPTED_AXIS {
			return Err(Error::InvalidOrMissingAxis("X, Y"));
		}
		let axis = self.axis;

		match (&self.start, &self.end_a, &self.end_b) {
			(None, _, _) => Err(Error::MissingPoint("starting point")),
			(_, None, _) => Err(Error::MissingPoint("first ending point")),
			(_, _, None) => Err(Error::MissingPoint("second ending point")),
			(Some(start), Some(end_a), Some(end_b)) => {
				if nth!(*end_a, axis) != nth!(*end_b, axis) {
					Err(match axis {
						0 => Error::InvalidX(end_a.0, end_b.0),
						1 => Error::InvalidY(end_a.1, end_b.1),
						_ => unreachable!()
					})
				} else {
					Ok(BresenhamZip::new(*start, *end_a, *end_b, self.axis))
				}
			}
		}
	}

}

#[cfg(test)]
mod test {
	use crate::Axis;
	use crate::error::Error;
	use crate::zip::Builder;

	macro_rules! build {
	    (x: $start:tt -> $end_a:tt, $end_b:tt) => {
		    Builder::new()
		      .axis(Axis::X)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
	    };
	    (y: $start:tt -> $end_a:tt, $end_b:tt) => {
		    Builder::new()
		      .axis(Axis::Y)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
	    };
	}

	#[test]
	fn invalid_axis() {
		let expected = Error::InvalidOrMissingAxis("X, Y");
		// Invalid
		assert_eq!(expected, Builder::<i32>::new().axis(Axis::Z).build().unwrap_err());
		// Missing
		assert_eq!(expected, Builder::<i32>::new().build().unwrap_err());
	}

	#[test]
	fn missing_point() {
		let builder = &mut Builder::new();
		builder.axis(Axis::X);
		// Missing starting point
		if let Err(err) = builder.build() {
			assert_eq!(err, Error::MissingPoint("starting point"));
		}
		// Missing first ending point
		builder.start_point((50, 50));
		if let Err(err) = builder.build() {
			assert_eq!(err, Error::MissingPoint("first ending point"));
		}
		// Missing first ending point
		builder.first_ending_point((0, 100));
		if let Err(err) = builder.build() {
			assert_eq!(err, Error::MissingPoint("second ending point"));
		}
	}

	#[test]
	fn invalid_points() {
		// Invalid X
		assert_eq!(build!(x: (50,50) -> (100,0), (0, 0)).unwrap_err(), Error::InvalidX(100, 0));
		// Invalid Y
		assert_eq!(build!(y: (50,50) -> (0,0), (0, 100)).unwrap_err(), Error::InvalidY(0, 100));
	}

	#[test]
	fn valid() {
		// Direct building
		assert_eq!(format!("{:?}", build!(x: (50, 50) -> (0, 0), (0, 100)).unwrap()),
		           "BresenhamZip [ (50, 50), (50, 50) ]. Goal: 0");
		// Modified building
		let built = Builder::new()
			.axis(Axis::X)
			.axis(Axis::Y)
			.start_point((25, 25))
			.second_ending_point((50, 50))
			.start_point((10, 10))
			.first_ending_point((0, 100))
			.second_ending_point((100, 100))
			.build();
		assert_eq!(format!("{:?}", built.unwrap()), "BresenhamZip [ (10, 10), (10, 10) ]. Goal: 100");
	}

}