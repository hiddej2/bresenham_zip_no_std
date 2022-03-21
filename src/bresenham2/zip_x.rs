use std::fmt::{Debug, Formatter};
use line_drawing::Bresenham;
use crate::error::Error;
use crate::{Point2, SignedNum};

/// Iterator to generate the lines along the X axis in 2D space.
/// Each iteration will yield two points with the same width.
///
/// # Example
/// ```
/// # use std::error::Error;
/// # use bresenham_zip::bresenham2::BresenhamZipX;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// for (top, bottom) in BresenhamZipX::new((50, 50), (100, 0), (100, 250))? {
///   println!("{:?} - {:?}", top, bottom);
///   assert_eq!(top.0, bottom.0);
///   assert!((0..=50).contains(&top.1));
///   assert!((50..=250).contains(&bottom.1));
/// }
/// #   Ok(())
/// # }
pub struct BresenhamZipX<T> {
	top: Bresenham<T>,
	bottom: Bresenham<T>,
	prev_top: Point2<T>,
	prev_bottom: Point2<T>,
	goal: T
}

impl<T: SignedNum> BresenhamZipX<T> {

	/// Builds the iterator to generate the lines advancing through the X axis. It needs three points,
	/// two of them with the same X value.
	///
	/// # Arguments
	/// * `start` - Starting point, it's the point with the divergent X.
	/// * `end_top`- One of the points sharing the X (it doesn't need to be at the top necessarily)
	/// * `end_bottom`- The other point sharing the X (it doesn't need to be at the bottom necessarily)
	///
	/// # Errors
	/// If the `end_top` and `end_bottom` points don't share the same X the build will be invalid and
	/// the result will be an error
	///
	#[inline]
	pub fn new(start: Point2<T>, end_top: Point2<T>, end_bottom: Point2<T>) -> Result<Self, Error<T>> {
		if end_top.0 != end_bottom.0 {
			return Err(Error::InvalidX(end_top.0, end_bottom.0))
		}

		Ok(Self {
			top: Bresenham::new(start, end_top),
			bottom: Bresenham::new(start, end_bottom),
			prev_top: start,
			prev_bottom: start,
			goal: end_top.0
		})
	}

}

impl<T: SignedNum> Iterator for BresenhamZipX<T> {
	type Item = (Point2<T>, Point2<T>);

	#[allow(clippy::while_let_on_iterator)]  // needs to be like that to keep using the iterator
	fn next(&mut self) -> Option<Self::Item> {
		let mut top = None;
		while let Some(point) = self.top.next() {
			if (point.0 - self.prev_top.0).abs() > T::zero() {
				top = Some(self.prev_top);
				self.prev_top = point;
				break;
			}
			self.prev_top = point;
		}

		let mut bottom = None;
		while let Some(point) = self.bottom.next() {
			if (point.0 - self.prev_bottom.0).abs() > T::zero() {
				bottom = Some(self.prev_bottom);
				self.prev_bottom = point;
				break;
			}
			self.prev_bottom = point;
		}

		if let Some(top_point) = top {
			Some((top_point, bottom.unwrap()))
		} else if self.prev_top.0 == self.goal {
				self.goal -= T::one();
				Some((self.prev_top, self.prev_bottom))
		} else { None }
	}
}

impl<T: SignedNum> Debug for BresenhamZipX<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "BresenhamZipX [
		previous_top_point: ({:?}, {:?}),
		previous_bottom_point: ({:?}, {:?})
		]", self.prev_top.0, self.prev_top.1, self.prev_bottom.0, self.prev_bottom.1)
	}
}

#[cfg(test)]
mod tests {
	use crate::bresenham2::BresenhamZipX;
	use crate::error::Error;

	#[test]
	fn invalid_x() {
		let result = BresenhamZipX::new((0,0), (1,1), (2,2));
		assert_eq!(result.unwrap_err(), Error::InvalidX(1,2));
	}

	#[test]
	fn symmetric() {
		let mut expected_top_y = 50;
		let mut expected_bottom_y = 50;
		let mut expected_x = 50;

		for (top, bottom) in BresenhamZipX::new((50, 50), (100, 0), (100, 100)).unwrap() {
			assert_eq!(expected_top_y, top.1);
			assert_eq!(expected_bottom_y, bottom.1);
			assert_eq!(expected_x, top.0);
			assert_eq!(top.0, bottom.0);

			expected_top_y -= 1;
			expected_bottom_y += 1;
			expected_x += 1;
		}
	}

	#[test]
	fn asymmetric() {
		let mut expected_top_y = 50;
		let mut expected_bottom_y = 50;
		let mut expected_x = 50;

		for (top, bottom) in BresenhamZipX::new((50, 50), (400, 0), (400, 800)).unwrap() {
			assert!(top.1 <= expected_top_y);
			assert!(bottom.1 >= expected_bottom_y);
			assert_eq!(expected_x, top.0);
			assert_eq!(top.0, bottom.0);

			expected_top_y = top.1;
			expected_bottom_y = bottom.1;
			expected_x += 1;
		}
	}

	#[test]
	fn inverted() {
		let mut expected_top_y = 50;
		let mut expected_bottom_y = 50;
		let mut expected_x = 50;

		for (top, bottom) in BresenhamZipX::new((50, 50), (0, 0), (0, 100)).unwrap() {
			assert_eq!(expected_top_y, top.1);
			assert_eq!(expected_bottom_y, bottom.1);
			assert_eq!(expected_x, top.0);
			assert_eq!(top.0, bottom.0);

			expected_top_y -= 1;
			expected_bottom_y += 1;
			expected_x -= 1;
		}
	}

}