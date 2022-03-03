pub mod bresenham;

pub trait SignedNum: line_drawing::SignedNum + std::fmt::Debug {}
impl<T: line_drawing::SignedNum + std::fmt::Debug> SignedNum for T {}