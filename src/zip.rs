//! Package with the logic of the two-dimensional BresenhamZip

mod builder;

use std::fmt::{Debug, Formatter};
use line_drawing::Bresenham;
use crate::{Point2, SignedNum};
use crate::util::Point;

pub use builder::Builder;

pub struct BresenhamZip<T> {
	a: Bresenham<T>,
	b: Bresenham<T>,
	prev_a: Point2<T>,
	prev_b: Point2<T>,
	goal: T,
	axis: u8
}

impl<T: SignedNum> BresenhamZip<T> {

	#[inline]
	pub(crate) fn new(start: Point2<T>, end1: Point2<T>, end2: Point2<T>, axis: u8) -> BresenhamZip<T> {
		Self {
			a: Bresenham::new(start, end1),
			b: Bresenham::new(start, end2),
			prev_a: start,
			prev_b: start,
			goal: end1.nth(axis),
			axis
		}
	}

}

impl<T: SignedNum> Iterator for BresenhamZip<T> {
	type Item = (Point2<T>, Point2<T>);

	#[allow(clippy::while_let_on_iterator)]  // needs to be like that to keep using the iterator
	fn next(&mut self) -> Option<Self::Item> {
		let axis = self.axis;

		let mut a = None;
		while let Some(point) = self.a.next() {
			if (point.nth(axis) - self.prev_a.nth(axis)).abs() > T::zero() {
				a = Some(self.prev_a);
				self.prev_a = point;
				break;
			}
			self.prev_a = point;
		}

		let mut b = None;
		while let Some(point) = self.b.next() {
			if (point.nth(axis) - self.prev_b.nth(axis)).abs() > T::zero() {
				b = Some(self.prev_b);
				self.prev_b = point;
				break;
			}
			self.prev_b = point;
		}

		if let Some(point) = a {
			Some((point, b.unwrap()))
		} else if self.prev_a.nth(axis) == self.goal {
			self.goal -= T::one();
			Some((self.prev_a, self.prev_b))
		} else { None }
	}

}

impl<T: SignedNum> Debug for BresenhamZip<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "BresenhamZip [ ({:?}, {:?}), ({:?}, {:?}) ]. Goal: {:?}",
		  self.prev_a.0, self.prev_a.1,
		  self.prev_b.0, self.prev_b.1,
			self.goal
		)
	}
}

#[cfg(test)]
mod tests {
	use super::BresenhamZip;

	macro_rules! symmetric {
	    ($a:tt, $b: tt, $c: tt, $axis: tt, $axis1: tt) => {
				let mut for_a = 50;
				let mut for_b = 50;
				let mut matching = 50;

				for (a, b) in BresenhamZip::new($a, $b, $c, $axis) {
			    assert_eq!(for_a, a.$axis1);
			    assert_eq!(for_b, b.$axis1);
					assert_eq!(matching, a.$axis);
					assert_eq!(matching, b.$axis);

					for_a -= 1;
					for_b += 1;
					matching += 1;
				}
	    };
	}

	macro_rules! asymmetric {
	    ($a: tt, $b: tt, $c: tt, $axis: tt, $axis1: tt) => {
				let mut for_a = 50;
				let mut for_b = 50;
				let mut matching = 50;

				for (a, b) in BresenhamZip::new($a, $b, $c, $axis) {
					assert!(a.$axis1 <= for_a);
					assert!(b.$axis1 >= for_b);
					assert_eq!(matching, a.$axis);
					assert_eq!(a.$axis, b.$axis);

					for_a = a.$axis1;
					for_b = b.$axis1;
					matching += 1;
				}
	    };
	}

	macro_rules! inverted {
	    ($a:tt, $b: tt, $c: tt, $axis: tt, $axis1: tt) => {
		    let mut for_a = 50;
				let mut for_b = 50;
				let mut matching = 50;

				for (a, b) in BresenhamZip::new($a, $b, $c, $axis) {
					assert_eq!(for_a, a.$axis1);
					assert_eq!(for_b, b.$axis1);
					assert_eq!(matching, a.$axis);
					assert_eq!(a.$axis, b.$axis);

					for_a -= 1;
					for_b += 1;
					matching -= 1;
				}
	    };
	}

	mod x_axis {
		use super::BresenhamZip;

		#[test]
		fn symmetric() {
			symmetric!((50, 50), (100, 0), (100, 100), 0, 1);
		}

		#[test]
		fn asymmetric() {
			asymmetric!((50, 50), (400, 0), (400, 800), 0, 1);
		}

		#[test]
		fn inverted() {
			inverted!((50, 50), (0, 0), (0, 100), 0, 1);
		}

	}

	mod y_axis {
		use super::BresenhamZip;

		#[test]
		fn symmetric() {
			symmetric!((50, 50), (0, 100), (100, 100), 1, 0);
		}

		#[test]
		fn asymmetric() {
			asymmetric!((50, 50), (0, 400), (800, 400), 1, 0);
		}

		#[test]
		fn inverted() {
			inverted!((50, 50), (0, 0), (100, 0), 1, 0);
		}

	}

}
