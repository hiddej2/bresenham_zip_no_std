use crate::{Point2, Point3, SignedNum};

macro_rules! nth {
    ($x:expr, $axis:tt) => {
	    match $axis {
		    0 => $x.0,
		    1 => $x.1,
		    _ => unreachable!()
	    }
    }
}

macro_rules! nth3 {
    ($x:expr, $axis:tt) => {
	    match $axis {
		    0 => $x.0,
		    1 => $x.1,
		    2 => $x.2,
		    _ => unreachable!()
	    }
    }
}

pub trait Point<T> {
	fn nth(&self, index: u8) -> T;
}

impl<T: SignedNum> Point<T> for Point2<T> {
	fn nth(&self, index: u8) -> T {
		nth!(self, index)
	}
}

impl<T: SignedNum> Point<T> for Point3<T> {
	fn nth(&self, index: u8) -> T {
		nth3!(self, index)
	}
}


