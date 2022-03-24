/// Convenience tool to build a whole zip of any dimension and axis with a single line call.
/// The schema call goes like this:
///
/// `<dimension>:<axis> - <starting point> -> <first ending point>, <second ending point>`
///
/// Being: `dimension` one of `2` or `3`, axis: `x`|`y`|`z` and the points tuples of [crate::Point2]
/// or [crate::Point3] depending of the dimension of the Zip to create.
///
/// # Example
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> { ///
/// // 2D Zip to iterate in X///
/// let x2 = bresenham_zip::build_zip!(2:x - (50, 50) -> (100, 0), (100, 100))?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! build_zip {
    (2:x - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::build_2d_zip!(($crate::Axis::X) - $start -> $end_a, $end_b)
    };
    (2:y - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::build_2d_zip!(($crate::Axis::Y) - $start -> $end_a, $end_b)
    };
}

#[macro_export]
macro_rules! build_2d_zip {
    ($axis:tt - $start:tt -> $end_a:tt, $end_b:tt) => {
		    $crate::zip::Builder::new()
		      .axis($axis)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
	    };
}
