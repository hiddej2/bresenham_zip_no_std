use std::fmt::{Debug, Formatter};
use line_drawing::Bresenham3d;
use crate::error::Error;
use crate::{Point3, SignedNum};

/// Iterator to generate the lines along the X axis in 3D space.
/// Each iteration will yield two points with the same width.
///
/// # Example
/// ```
/// # use std::error::Error;
/// # use bresenham_zip::bresenham3::Bresenham3ZipX;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// for (top, bottom) in Bresenham3ZipX::new((50, 50, 50), (100, 0, 10), (100, 250, 200))? {
///   println!("{:?} - {:?}", top, bottom);
///   assert_eq!(top.0, bottom.0);
///   assert!((0..=50).contains(&top.1));
///   assert!((10..=50).contains(&top.2));
///   assert!((50..=250).contains(&bottom.1));
///   assert!((50..=200).contains(&bottom.2));
/// }
/// #   Ok(())
/// # }
pub struct Bresenham3ZipX<T> {
	top: Bresenham3d<T>,
	bottom: Bresenham3d<T>,
	prev_top: Point3<T>,
	prev_bottom: Point3<T>,
	goal: T
}

impl<T: SignedNum> Bresenham3ZipX<T> {

	/// Builds the iterator to generate the lines advancing through the X axis. It needs three points,
	/// two of them with the same X value.
	///
	/// # Arguments
	/// * `start` - Starting point, it's the point with the divergent X.
	/// * `end_top`- One of the points sharing the Y (it doesn't need to be at the top necessarily)
	/// * `end_bottom`- The other point sharing the Y (it doesn't need to be at the bottom necessarily)
	///
	/// # Errors
	/// If the `end_top` and `end_bottom` points don't share the same X the build will be invalid and
	/// the result will be an error
	///
	#[inline]
	pub fn new(start: Point3<T>, end_top: Point3<T>, end_bottom: Point3<T>) -> Result<Self, Error<T>> {
		if end_top.0 != end_bottom.0 {
			return Err(Error::InvalidX(end_top.0, end_bottom.0))
		}

		Ok(Self {
			top: Bresenham3d::new(start, end_top),
			bottom: Bresenham3d::new(start, end_bottom),
			prev_top: start,
			prev_bottom: start,
			goal: end_top.0
		})
	}

}

impl<T: SignedNum> Iterator for Bresenham3ZipX<T> {
	type Item = (Point3<T>, Point3<T>);

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

impl<T: SignedNum> Debug for Bresenham3ZipX<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Bresenham3ZipX [
		previous_top_point: ({:?}, {:?}, {:?}),
		previous_bottom_point: ({:?}, {:?}, {:?})
		]",
		       self.prev_top.0, self.prev_top.1, self.prev_top.2,
		       self.prev_bottom.0, self.prev_bottom.1, self.prev_bottom.2
		)
	}
}

#[cfg(test)]
mod tests {
	use crate::bresenham3::Bresenham3ZipX;
	use crate::error::Error;

	#[test]
	fn invalid_x() {
		let result = Bresenham3ZipX::new((0,0,0), (1,1,1), (2,2,2));
		assert_eq!(result.unwrap_err(), Error::InvalidX(1,2));
	}

	#[test]
	fn symmetric() {
		let mut expected_top_y_z = 50;
		let mut expected_bottom_y_z = 50;
		let mut expected_x = 50;

		for (top, bottom) in Bresenham3ZipX::new((50, 50, 50), (100, 0, 0), (100, 100, 100)).unwrap() {
			assert_eq!(expected_top_y_z, top.1);
			assert_eq!(expected_top_y_z, top.2);
			assert_eq!(expected_bottom_y_z, bottom.1);
			assert_eq!(expected_bottom_y_z, bottom.2);
			assert_eq!(expected_x, top.0);
			assert_eq!(top.0, bottom.0);

			expected_top_y_z -= 1;
			expected_bottom_y_z += 1;
			expected_x += 1;
		}
	}

	#[test]
	fn asymmetric() {
		let mut expected_top_y = 50;
		let mut expected_top_z = 50;
		let mut expected_bottom_y = 50;
		let mut expected_bottom_z = 50;
		let mut expected_x = 50;

		for (top, bottom) in Bresenham3ZipX::new((50, 50, 50), (400, 0, 10), (400, 800, 200)).unwrap() {
			assert!(top.1 <= expected_top_y);
			assert!(top.2 <= expected_top_z);
			assert!(bottom.1 >= expected_bottom_y);
			assert!(bottom.2 >= expected_bottom_z);
			assert_eq!(expected_x, top.0);
			assert_eq!(top.0, bottom.0);

			expected_top_y = top.1;
			expected_top_z = top.2;
			expected_bottom_y = bottom.1;
			expected_bottom_z = bottom.2;
			expected_x += 1;
		}
	}

	#[test]
	fn inverted() {
		let mut expected_top_y_z = 50;
		let mut expected_bottom_y_z = 50;
		let mut expected_x = 50;

		for (top, bottom) in Bresenham3ZipX::new((50, 50, 50), (0, 0, 0), (0, 100, 100)).unwrap() {
			assert_eq!(expected_top_y_z, top.1);
			assert_eq!(expected_top_y_z, top.2);
			assert_eq!(expected_bottom_y_z, bottom.1);
			assert_eq!(expected_bottom_y_z, bottom.2);
			assert_eq!(expected_x, top.0);
			assert_eq!(top.0, bottom.0);

			expected_top_y_z -= 1;
			expected_bottom_y_z += 1;
			expected_x -= 1;
		}
	}

}