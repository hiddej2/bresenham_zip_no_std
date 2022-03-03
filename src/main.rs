use bresenham_zip::bresenham::BresenhamZipY;

fn main() {
    let zip: BresenhamZipY<isize> = BresenhamZipY::new((50, 50), (0, 100), (100, 0));
    println!("Zip: {:?}", zip);
}
