use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use crate::SignedNum;

#[derive(PartialEq)]
pub enum Error<T> {
	InvalidX(T,T),
	InvalidY(T,T),
}

impl<T: SignedNum> Error<T> {

	fn message(&self) -> String {
		match self {
			Error::InvalidX(left, right) => format!("Invalid Y. Both values must have the same Y ({left:?} != {right:?})"),
			Error::InvalidY(left, right) => format!("Invalid Y. Both values must have the same Y ({left:?} != {right:?})"),
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