use bresenham_zip::bresenham::BresenhamZipY;

fn main() {
    for (left, right) in BresenhamZipY::new((50, 50), (0, 100), (100, 100)) {
        println!("{:?}, {:?}", left, right);
    }
}
