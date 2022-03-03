use std::fmt::{Debug, Formatter};
use line_drawing::{Bresenham, Point};
use crate::bresenham::error::Error;
use crate::SignedNum;

pub struct BresenhamZipY<T> {
	left: Bresenham<T>,
	right: Bresenham<T>,
	prev_left: Point<T>,
	prev_right: Point<T>,
	goal: T
}

impl<T: SignedNum> BresenhamZipY<T> {

	#[inline]
	pub fn new(start: Point<T>, end_left: Point<T>, end_right: Point<T>) -> Result<Self, Error<T>> {
		if end_left.1 != end_right.1 {
			return Err(Error::InvalidY(end_left.1, end_right.1))
		}

		Ok(Self {
			left: Bresenham::new(start, end_left),
			right: Bresenham::new(start, end_right),
			prev_left: start,
			prev_right: start,
			goal: end_left.1
		})
	}

}

impl<T: SignedNum> Iterator for BresenhamZipY<T> {
	type Item = (Point<T>, Point<T>);

	fn next(&mut self) -> Option<Self::Item> {
		let mut left = None;
		while let Some(point) = self.left.next() {
			if (point.1 - self.prev_left.1).abs() > T::zero() {
				left = Some(self.prev_left);
				self.prev_left = point;
				break;
			}
			self.prev_left = point;
		}

		let mut right = None;
		while let Some(point) = self.right.next() {
			if (point.1 - self.prev_right.1).abs() > T::zero() {
				right = Some(self.prev_right);
				self.prev_right = point;
				break;
			}
			self.prev_right = point;
		}

		if left.is_some() && right.is_some() {
			Some((left.unwrap(), right.unwrap()))
		} else if self.prev_left.1 == self.goal {
				self.goal -= T::one();
				Some((self.prev_left, self.prev_right))
		} else { None }
	}
}

impl<T: SignedNum> Debug for BresenhamZipY<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "BresenhamZip [
		previous_left_point: ({:?}, {:?}),
		previous_right_point: ({:?}, {:?})
		]", self.prev_left.0, self.prev_left.1, self.prev_right.0, self.prev_right.1)
	}
}

#[cfg(test)]
mod tests {
	use crate::bresenham::BresenhamZipY;
	use crate::bresenham::error::Error;

	#[test]
	fn invalid_y() {
		let result = BresenhamZipY::new((0,0), (1,1), (2,2));
		assert_eq!(result.unwrap_err(), Error::InvalidY(1,2));
	}

	#[test]
	fn symmetric() {
		let mut expected_left_x = 50;
		let mut expected_right_x = 50;
		let mut expected_y = 50;

		for (left, right) in BresenhamZipY::new((50, 50), (0, 100), (100, 100)).unwrap() {
			assert_eq!(expected_left_x, left.0);
			assert_eq!(expected_right_x, right.0);
			assert_eq!(expected_y, left.1);
			assert_eq!(left.1, right.1);

			expected_left_x -= 1;
			expected_right_x += 1;
			expected_y += 1;
		}
	}

	#[test]
	fn asymmetric() {
		let mut expected_left_x = 50;
		let mut expected_right_x = 50;
		let mut expected_y = 50;

		for (left, right) in BresenhamZipY::new((50, 50), (0, 400), (800, 400)).unwrap() {
			assert!(left.0 <= expected_left_x);
			assert!(right.0 >= expected_right_x);
			assert_eq!(expected_y, left.1);
			assert_eq!(left.1, right.1);

			expected_left_x = left.0;
			expected_right_x = right.0;
			expected_y += 1;
		}
	}

}