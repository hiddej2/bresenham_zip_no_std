//! Library errors

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use crate::SignedNum;

/// Different errors that can happen using the library
#[derive(PartialEq)]
pub enum Error<T> {
	/// The last two points passed to build the BresenhamZip for X axis doesn't share the same X
	InvalidX(T,T),
	/// The last two points passed to build the BresenhamZip for Y axis doesn't share the same Y
	InvalidY(T,T),
	/// The last two points passed to build the BresenhamZip for Z axis doesn't share the same Z
	InvalidZ(T,T),
}

impl<T: SignedNum> Error<T> {

	fn message(&self) -> String {
		match self {
			Error::InvalidX(left, right) => format!("Invalid X. Both values must have the same X ({left:?} != {right:?})"),
			Error::InvalidY(left, right) => format!("Invalid Y. Both values must have the same Y ({left:?} != {right:?})"),
			Error::InvalidZ(left, right) => format!("Invalid Z. Both values must have the same Y ({left:?} != {right:?})"),
		}
	}

}

impl<T: SignedNum> Debug for Error<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message())
	}
}

impl<T: SignedNum> Display for Error<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message())
	}
}

impl<T: SignedNum> StdError for Error<T> {}