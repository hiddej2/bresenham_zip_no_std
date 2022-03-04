[![Crates.io](https://img.shields.io/crates/v/bresenham_zip)](https://crates.io/crates/bresenham_zip)
[![docs.rs](https://img.shields.io/docsrs/bresenham_zip)](https://doc.rs/bresenham_zip)

# Bresenham::zip

This library provides a wrapper to handle the simultaneous generation of two lines using [**Bresenham line algorithm**](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
This is something basic to implement [triangle rasterization](http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html)
using Bresenham. The wrapper works using the Bresenham algorithm of the [line drawing](https://crates.io/crates/line_drawing) crate. 

The provided BresenhamZip iterator will provide two points of the same `x` or `y` value at the same time conforming the 
longest possible line between both. This way each tuple provided will contain the starting and ending point of each
horizontal or vertical line conforming the triangle.

This crate was born during the development of [FerruX Canvas](https://crates.io/crates/ferrux_canvas) to manage the
generation of the points for the triangle filling function.

## Usage

Build the BresenhamZip specifying the three points of the triangle. 
The last two of them must share the same value for X or Y depending on which BresenhamZip you're using.

Each iteration will yield a pair of points defining the start and end of an horizontal or vertical line inside the triangle.

```rust
/// Rasterizes a triangle with an horizontal base
for (left, right) in BresenhamZipY::new((50, 50), (0, 100), (250, 100))? {
  draw_line(left, right); // draw_line is a figurative method, use one of your project
}
```

## Incoming

* BresenhamZipX to generate the pairs with steps along the X axis

## License

Licensed, at your option, under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.