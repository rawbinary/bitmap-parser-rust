# Bitmap Parser

A Rust implementation of Bitmap Parser.

**See Demo (Typescript Web version) at: https://bitmap-parser.robin.md**

<img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="200px" style="float: right; margin: 20px;"/>

## Why?

Well, previously I did make a bitmap parser in Typescript. Now, I thought to myself, why not make it in Rust.

**No any thirdparty library is used for any image parsing work, everything is fully self-coded for the sake of understanding every aspect of image parsing.**

## Bitmap Parsing

All the bitmap parsing related codes are in `bitmap.rs` file. Other folders are for using and running the lib.

### Structure of Bitmap

Structure of a bitmap file can be pretty different based of the types. Here, we only use the important bits of structure we need.

```rs
struct BitmapFileHeader {
    signature: u16,             // 2 bytes - BMP File Signature
    size: u32,                  // 4 bytes - Size of BMP File
    reserved: u32,              // 4 bytes = Reserved buffer generally used by application creating the image
    bits_offset: u32,           // 4 bytes = Offset at which the pixel data starts
}

struct BitmapImageHeader {
    header_size: u32,           // 4 bytes - The size of this header
    width: i32,                 // 4 bytes - Width of the Image in pixels
    height: i32,                // 4 bytes - Height of the Image in pixels
    color_planes: u16,          // 2 bytes - Number of color planes
    bit_depth: u16,             // 2 bytes - Number of bits per pixel; usually 8, 16, 24 and 32
    compression_method: u32,    // 4 bytes - Compression Method used
    image_size: u32,            // 4 bytes - Size of the bitmap pixel data
    pix_per_meter_x: i32,       // 4 bytes - Horizontal res; pixel per meter (signed int)
    pix_per_meter_y: i32,       // 4 bytes - Vertical res; pixel per meter (signed int)
    color_count: u32,           // 4 bytes - no. of colors in color palette
    imp_color_count: u32,       // 4 bytes - no. of imp. colors; usually ignored
    // other unused headers...
}
```

#### **Bitmap File Header**

The first 14 bytes of the BMP file contains File Headers.

#### **Bitmap Image Header**

DIB Header contains detailed information about the image, which is used to parse and display image properly. The size of this header differs from version and type of the BMP. The first 4 bytes of this header tells the size of the header.

#### **Pixel Data**

After reading the required data from the header, we skip all other headers to directly read from the pixel data offset we received from `BitmapFile.bits_offset` value.

The Pixel data is a block of 32-bit DWORDs. Usually pixels are stored "bottom-up", starting from lower-left corner, going from left to right. So, at the end of everything, we reverse the pixel array to make it straight.

Padding bytes must be appended to the end of the rows in order to bring up the length of the rows to a multiple of four bytes. When the pixel array is loaded into memory, each row must begin at a memory address that is a multiple of 4. This address/offset restriction is mandatory only for pixel arrays loaded in memory. For file storage purposes, only the size of each row must be a multiple of 4 bytes while the file offset can be arbitrary. A 24-bit bitmap with Width=1, would have 3 bytes of data per row (blue, green, red) and 1 byte of padding, while Width=2 would have 6 bytes of data and 2 bytes of padding, Width=3 would have 9 bytes of data and 3 bytes of padding, and Width=4 would have 12 bytes of data and no padding.

## References

[Bitmap File Structure, at digicamsoft.com](https://www.digicamsoft.com/bmp/bmp.html)

[The BMP File Format, Part 1 By David Charlap at Dr. Dobb's journal of software tools (drdobbs.com), March 1995](https://drdobbs.com/architecture-and-design/the-bmp-file-format-part-1/184409517)
