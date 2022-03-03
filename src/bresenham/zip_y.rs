use std::fmt::{Debug, Formatter};
use line_drawing::{Bresenham, SignedNum};
use crate::Point;

//#[derive(Debug)]
pub struct BresenhamZipY<T> {
	left: Bresenham<T>,
	right: Bresenham<T>
}

impl<T: SignedNum> BresenhamZipY<T> {

	#[inline]
	pub fn new(start: Point<T>, end_left: Point<T>, end_right: Point<T>) -> Self {
		// TODO check ending points -> throw error if they don't share the same Y
		Self {
			left: Bresenham::new(start, end_left),
			right: Bresenham::new(start, end_right)
		}
	}

}

impl<T> Debug for BresenhamZipY<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "BresenhamZip")
	}
}