mod bitmap;

fn main() {
    println!("Robin's Bitmap Loader");
    bitmap::parse("assets/24bituncompressed.bmp");
}
