[![Crates.io](https://img.shields.io/crates/v/bresenham_zip)](https://crates.io/crates/bresenham_zip)
[![docs.rs](https://img.shields.io/docsrs/bresenham_zip)](https://doc.rs/bresenham_zip)

# Bresenham::zip with no_std support

DISCLAIMER: this is a quick and dirty removal of all ```std``` dependencies, so you might find commented out code and crashes where previously, it would've given a nice error.

Also added a no_std fork of the ```line_drawing``` in ```Cargo.toml```.




This library provides a wrapper to handle the simultaneous generation of two lines using [**Bresenham line algorithm**](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
This is something basic to implement [triangle rasterization](http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html)
using Bresenham. The wrapper works using the Bresenham algorithm of the [line drawing](https://crates.io/crates/line_drawing) crate. 

The provided BresenhamZip iterator will provide two points of the same value in the specified axis at the same time conforming the 
longest possible line between both. This way each tuple provided will contain the starting and ending point of each
horizontal or vertical line conforming the triangle.

This crate was born during the development of [FerruX Canvas](https://crates.io/crates/ferrux_canvas) to manage the
generation of the points for the triangle filling function.

## Usage

Build the BresenhamZip using the builders or the helper macro.
You will need to specify the axis to navigate and the three points of the triangle. 
The last two of them must share the same value for X, Y or Z depending on which BresenhamZip you're using.

Then iterate with the Zip with your method of choice. Each iteration will yield a pair of points defining the start and end of an horizontal or vertical line inside the triangle.

```rust
/// Rasterizes a 2D triangle with an horizontal base
for (left, right) in Builder::new().axis(bresenham_zip::Axis::Y)
        .start_point((50, 50)).first_ending_point((0, 100)).second_ending_point((250, 100)).build()? {
    draw_line(top, bottom); // draw_line is a figurative method, use one of your project
    assert_eq!(left.1, right.1);
    assert!((0..=50).contains(&left.0));
    assert!((50..=250).contains(&right.0));
 }
```

```rust
/// Rasterizes a 3D triangle using the helper macro
for (a, b) in build_zip!(3D:Z - (50, 50, 50) -> (0, 10, 200), (100, 250, 200))? {
    println!("{:?} - {:?}", a, b);
    assert_eq!(a.2, b.2);
    assert!((0..=50).contains(&a.0));
    assert!((10..=50).contains(&a.1));
    assert!((50..=100).contains(&b.0));
    assert!((50..=250).contains(&b.1));
}
```

## License

Licensed, at your option, under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
