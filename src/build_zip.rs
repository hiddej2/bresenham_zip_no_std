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
/// // 2D Zip to iterate through X
/// let zip_2d_x = bresenham_zip::build_zip!(2D:X - (50, 50) -> (100, 0), (100, 100))?;
/// // 3D Zip to iterate through Z
/// let zip_3d_z = bresenham_zip::build_zip!(3D:Z - (50, 50, 50) -> (0, 0, 100), (25, 50, 100))?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! build_zip {
    (2D:X - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::zip::Builder::new()
		      .axis($crate::Axis::X)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
    };
    (2D:Y - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::zip::Builder::new()
		      .axis($crate::Axis::Y)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
    };
    (3D:X - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::zip_3d::Builder3d::new()
		      .axis($crate::Axis::X)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
    };
    (3D:Y - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::zip_3d::Builder3d::new()
		      .axis($crate::Axis::Y)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
    };
    (3D:Z - $start:tt -> $end_a:tt, $end_b:tt) => {
	    $crate::zip_3d::Builder3d::new()
		      .axis($crate::Axis::Z)
					.start_point($start)
					.first_ending_point($end_a)
					.second_ending_point($end_b)
					.build()
    };
}