#![allow(unused)]

mod error;

use std::{
    fmt::format,
    io::{BufReader, Read, Seek, SeekFrom},
};

/**
 * Robin's Bitmap Parser
 *
 * Supported Compression: BI_RGB (Uncompressed)
 * Supported Bit Depth: 24 and 32 bit
 *
 * Warning: Not to be used in any production environment without prior tests.
 */
use crate::error::Error;
use byteorder::{ByteOrder, LittleEndian};
use hex;

#[derive(Debug)]
struct BitmapFileHeader {
    signature: u16,
    size: u32,
    reserved: u32,
    bits_offset: u32,
}

#[derive(Debug)]
struct BitmapImageHeader {
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
}

#[derive(Debug)]
pub struct BitmapFile {
    file_header: BitmapFileHeader,
    image_header: BitmapImageHeader, // other....
                                     // pixel_data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
enum CompressionMethod {
    BiRgb,
    BiRle8,
    BiRle4,
    BiBitfields,
    BiJpeg,
    BiPng,
    BiAlphabitfields,
    BiCmyk = 11,
    BiCmykrle8 = 12,
    BiCmykrle4 = 13,
}

#[derive(Debug)]
struct BGRA {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8,
}

impl BGRA {
    pub fn from_buffer(buf: &[u8]) -> Result<BGRA, Error> {
        match buf.len() {
            4 => Ok(BGRA {
                blue: buf[0],
                green: buf[1],
                red: buf[2],
                alpha: buf[3],
            }),
            n => Err(Error::InvalidRange(n)),
        }
    }
}

const SUPPORTED_COMPRESSION: [u32; 2] = [
    CompressionMethod::BiRgb as u32,
    CompressionMethod::BiRle8 as u32,
];
const SUPPORTED_BIT_DEPTH: [u16; 3] = [8, 16, 24];

impl BitmapFile {
    pub fn load(filename: &str) -> Result<BitmapFile, Error> {
        let mut file = std::fs::File::open(filename).unwrap();

        let mut reader = BufReader::new(file.try_clone().unwrap());

        let mut load_part = |size| {
            let mut buf = Vec::with_capacity(size);

            // Get a reader for the next `size` amount of bytes
            let mut part_reader = (&mut reader).take(size as u64);

            // Read the part into the buffer
            part_reader.read_to_end(&mut buf).unwrap();
            buf
        };

        let bmp_file_header = BitmapFileHeader {
            signature: LittleEndian::read_u16(&load_part(2)),
            size: LittleEndian::read_u32(&load_part(4)),
            reserved: LittleEndian::read_u32(&load_part(4)),
            bits_offset: LittleEndian::read_u32(&load_part(4)),
        };

        let bmp_sig: u16 = 0x4d42;
        if (bmp_file_header.signature != bmp_sig) {
            return Err(Error::InvalidSignature);
        }

        let bmp_image_header = BitmapImageHeader {
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
        };

        if (!SUPPORTED_COMPRESSION.contains(&bmp_image_header.compression_method)) {
            return Err(Error::UnsupportedCompression);
        }

        if (!SUPPORTED_BIT_DEPTH.contains(&bmp_image_header.bit_depth)) {
            return Err(Error::UnsupportedBitDepth);
        }

        // Color Tables

        // Since color tables start after headers and headers can differ from types, we seek to end of header
        file.seek(SeekFrom::Start(14 + bmp_image_header.header_size as u64));

        let mut color_table_size = 0;

        if (bmp_image_header.bit_depth == 1) {
            color_table_size = 2;
        } else if (bmp_image_header.bit_depth == 4) {
            color_table_size = 16;
        } else if (bmp_image_header.bit_depth == 8) {
            color_table_size = 256;
        }

        let color_table: Vec<BGRA> = Vec::new();

        if (color_table_size != 0) {
            let color_table: Vec<BGRA> = load_part(4 * bmp_image_header.color_count as usize)
                .chunks(4)
                .map(|x| BGRA::from_buffer(x).unwrap())
                .collect();
        }

        let bitmap_size = bmp_image_header.width * bmp_image_header.height;

        let line_width =
            ((bmp_image_header.width * bmp_image_header.bit_depth as i32 / 8) + 3) & !3;

        file.seek(SeekFrom::Start(bmp_file_header.bits_offset as u64));

        if (bmp_image_header.compression_method == CompressionMethod::BiRgb as u32) {
            // Pixel Array
            let pixel_data_size = bmp_image_header.image_size as usize;
            if (pixel_data_size <= 0) {
                return Err(Error::InvalidPixelData);
            }
            let pixel_data: Vec<u8> = load_part(pixel_data_size);
            // let hex = |bytes: Vec<u8>| {
            //     let str = bytes
            //         .chunks(3)
            //         .map(|b| format!("{:02x}", b.to_vec()).to_string())
            //         .collect::<Vec<String>>()
            //         .join(" ");
            //     str
            // };

            let pixel_array: Vec<Vec<String>> = pixel_data
                .chunks(line_width as usize)
                .map(|line| -> Vec<String> {
                    hex::encode_upper(line)
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(6)
                        .filter_map(|c| {
                            let pix = c.iter().collect::<String>();
                            if (pix.len() == 6) {
                                return Some(format!("#{0}", pix));
                            } else {
                                return None;
                            }
                        })
                        .collect::<Vec<String>>()
                })
                .collect();

            dbg!(pixel_array);

            // for i in 0..bmp_image_header.height - 1 {
            //     let line = load_part(line_width as usize);
            //     let line_ptr = line.as_ptr();

            //     // for j in 0..bmp_image_header.width - 1 {
            //     //     if (bmp_image_header.bit_depth == 24) {
            //     //         println!("{:#04X?}", );
            //     //     }
            //     // }
            //     line.chunks(3);
            //     println!("{:#04X?}", line);
            // }
        } else {
            println!("LOL")
        }
        //     pixel_data = load_part(pixel_data_size).to_vec();
        // } else {

        // dbg!(pixel_data);

        Ok(BitmapFile {
            file_header: bmp_file_header,
            image_header: bmp_image_header,
        })
    }
}
