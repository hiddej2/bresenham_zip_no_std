# Bresenham::zip

This library provides a wrapper to handle the generation of two lines using **Bresenham** at the same time.
This is something that is basic to [triangle rasterization](http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html)
using Bresenham. The wrapper works using the  **Bresenham** algorithm of the [line drawing](https://crates.io/crates/line_drawing) crate. 

The provided BresenhamZip iterator will provide two points of the same `x` or `y` value at the same time conforming the 
longest possible line between both. This way each tuple provided will contain the starting and ending point of each
horizontal or vertical line conforming the triangle.

This crate was born during the development of [FerruX Canvas](https://crates.io/crates/ferrux_canvas) to manage the
generation of the points for the triangle filling function.