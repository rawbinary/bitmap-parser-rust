#![allow(unused)]

/**
 * Robin's Bitmap Parser
 *
 * Supported Compression: BI_RGB (Uncompressed)
 * Supported Bit Depth: 24 and 32 bit
 *
 * Warning: Not to be used in any production environment without prior tests.
 */
use std::{
    fs,
    io::{BufReader, Read},
};

use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
struct BitmapFile {
    signature: u16,
    size: u32,
    reserved: u32,
    bits_offset: u32,

    header_size: u32,
    width: i32,
    height: i32,
    color_planes: u16,
    bit_depth: u16,
    compression_method: u32,
    image_size: u32,
    pix_per_meter_x: i32,
    pix_per_meter_y: i32,
    color_count: u32,
    imp_color_count: u32,

    // other....
    pixel_data: Vec<u8>,
}

impl BitmapFile {
    fn load<T: Read>(reader: &mut T) -> BitmapFile {
        let mut load_part = |size| {
            let mut buf = Vec::with_capacity(size);
            // Get a reader for the next `size` amount of bytes
            let mut part_reader = reader.take(size as u64);

            // Read the part into the buffer
            part_reader.read_to_end(&mut buf).unwrap();

            buf
        };

        BitmapFile {
            signature: LittleEndian::read_u16(&load_part(2)),
            size: LittleEndian::read_u32(&load_part(4)),
            reserved: LittleEndian::read_u32(&load_part(4)),
            bits_offset: LittleEndian::read_u32(&load_part(4)),

            header_size: LittleEndian::read_u32(&load_part(4)),
            width: LittleEndian::read_i32(&load_part(4)),
            height: LittleEndian::read_i32(&load_part(4)),
            color_planes: LittleEndian::read_u16(&load_part(2)),
            bit_depth: LittleEndian::read_u16(&load_part(2)),
            compression_method: LittleEndian::read_u32(&load_part(4)),
            image_size: LittleEndian::read_u32(&load_part(4)),
            pix_per_meter_x: LittleEndian::read_i32(&load_part(4)),
            pix_per_meter_y: LittleEndian::read_i32(&load_part(4)),
            color_count: LittleEndian::read_u32(&load_part(4)),
            imp_color_count: LittleEndian::read_u32(&load_part(4)),

            pixel_data: Vec::new(),
        }
    }
}

/// Parse a bitmap image file
pub(crate) fn parse(filename: &str) {
    let file = fs::File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let bmp = BitmapFile::load(&mut reader);

    let bmp_sig: u16 = 0x4d42;
    if (bmp.signature != bmp_sig) {
        println!("Not a valid bitmap signature.");
    }

    dbg!(bmp);
}
