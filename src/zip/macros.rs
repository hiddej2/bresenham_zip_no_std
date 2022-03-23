#[macro_export]
macro_rules! nth {
    ($x:expr, $axis:tt) => {
	    match $axis {
		    0 => $x.0,
		    1 => $x.1,
		    _ => unreachable!()
	    }
    }
}