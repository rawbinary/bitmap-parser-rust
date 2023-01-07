#![allow(unused)]
// #![windows_subsystem = "windows"]
use bitmap::BitmapFile;
use std::env;

// mod window;
use pixel_canvas::{input::MouseState, Canvas, Color};

fn main() {
    let filename: String = env::args().collect::<Vec<String>>().pop().unwrap();

    let bmp = BitmapFile::load(&filename).unwrap();

    let canvas = Canvas::new(
        bmp.image_header.width as usize,
        bmp.image_header.height as usize,
    )
    .title("Robin's Bitmap Loader")
    // .show_ms(true)
    .state(MouseState::new())
    .input(MouseState::handle_input);

    // dbg!(bmp.image_header);
    let pixel_array = bmp.pixel_array;

    canvas.render(move |mouse, image| {
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = Color {
                    r: pixel_array[y][x].red,
                    g: pixel_array[y][x].green,
                    b: pixel_array[y][x].blue,
                }
            }
        }
    });
}
