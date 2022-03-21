use std::fmt::{Debug, Formatter};
use line_drawing::Bresenham3d;
use crate::error::Error;
use crate::{Point3, SignedNum};

macro_rules! nth {
    ($x:expr, $axis:tt) => {
	    match $axis {
		    0 => $x.0,
		    1 => $x.1,
		    2 => $x.2,
		    _ => unreachable!()
	    }
    }
}

pub struct Bresenham3dZip<T> {
	a: Bresenham3d<T>,
	b: Bresenham3d<T>,
	prev_a: Point3<T>,
	prev_b: Point3<T>,
	goal: T,
	axis: u8
}

impl<T: SignedNum> Bresenham3dZip<T> {

	#[inline]
	pub fn new(start: Point3<T>, end1: Point3<T>, end2: Point3<T>, axis: u8) -> Result<Self, Error<T>> {
		if nth!(end1, axis) != nth!(end2, axis) {
			return Err(Error::InvalidX(nth!(end1, axis), nth!(end2, axis)))
		}

		Ok(Self {
			a: Bresenham3d::new(start, end1),
			b: Bresenham3d::new(start, end2),
			prev_a: start,
			prev_b: start,
			goal: nth!(end1, axis),
			axis
		})
	}

}

impl<T: SignedNum> Iterator for Bresenham3dZip<T> {
	type Item = (Point3<T>, Point3<T>);

	#[allow(clippy::while_let_on_iterator)]  // needs to be like that to keep using the iterator
	fn next(&mut self) -> Option<Self::Item> {
		let axis = self.axis;

		let mut a = None;
		while let Some(point) = self.a.next() {
			if (nth!(point, axis) - nth!(self.prev_a, axis)).abs() > T::zero() {
				a = Some(self.prev_a);
				self.prev_a = point;
				break;
			}
			self.prev_a = point;
		}

		let mut b = None;
		while let Some(point) = self.b.next() {
			if (nth!(point, axis) - nth!(self.prev_b, axis)).abs() > T::zero() {
				b = Some(self.prev_b);
				self.prev_b = point;
				break;
			}
			self.prev_b = point;
		}

		if let Some(point) = a {
			Some((point, b.unwrap()))
		} else if nth!(self.prev_a, axis) == self.goal {
			self.goal -= T::one();
			Some((self.prev_a, self.prev_b))
		} else { None }
	}
}

impl<T: SignedNum> Debug for Bresenham3dZip<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Bresenham3ZipX [
			({:?}, {:?}, {:?}),
			({:?}, {:?}, {:?})
		]",
		  self.prev_a.0, self.prev_a.1, self.prev_a.2,
		  self.prev_b.0, self.prev_b.1, self.prev_b.2
		)
	}
}

#[cfg(test)]
mod tests {
	use super::Bresenham3dZip;
	use crate::error::Error;

	#[test]
	fn invalid_x() {
		let result = Bresenham3dZip::new((0, 0, 0), (1, 1, 1), (2, 2, 2), 0);
		assert_eq!(result.unwrap_err(), Error::InvalidX(1, 2));
	}

	#[test]
	fn symmetric() {
		let mut expected_top_y_z = 50;
		let mut expected_bottom_y_z = 50;
		let mut expected_x = 50;

		for (top, bottom) in Bresenham3dZip::new((50, 50, 50), (100, 0, 0), (100, 100, 100), 0).unwrap() {
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

		for (top, bottom) in Bresenham3dZip::new((50, 50, 50), (400, 0, 10), (400, 800, 200), 0).unwrap() {
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

		for (top, bottom) in Bresenham3dZip::new((50, 50, 50), (0, 0, 0), (0, 100, 100), 0).unwrap() {
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
