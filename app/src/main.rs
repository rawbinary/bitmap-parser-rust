use bitmap::BitmapFile;

fn main() {
    println!("Robin's Bitmap Loader");

    let _ = BitmapFile::load("app/assets/24bituncompressed.bmp").unwrap();

    // dbg!(bmp);
}
