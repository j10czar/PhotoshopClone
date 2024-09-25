use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use tinytga::{Bpp, Compression, DataType, ImageOrigin, RawPixel, RawTga, TgaHeader};


fn main(){

    // Include an image from a local path as bytes.
    let data = include_bytes!("../input/car.tga");

    // Create a TGA instance from a byte slice.
    let img = RawTga::from_slice(data).unwrap();

    // Take a look at the raw image header.
    //assert_eq!(
    //    img.header(),
    //    TgaHeader {
    //        id_len: 0,
    //        has_color_map: false,
    //        data_type: DataType::TrueColor,
    //        compression: Compression::Rle,
    //        color_map_start: 0,
    //        color_map_len: 0,
    //        color_map_depth: None,
    //        x_origin: 0,
    //        y_origin: 4,
    //        width: 4,
    //        height: 4,
    //        pixel_depth: Bpp::Bits24,
    //        image_origin: ImageOrigin::TopLeft,
    //        alpha_channel_depth: 0,
    //    }
    //);
    //collect the images raw pixels into a vector
    let pixels: Vec<_> = img.pixels().collect();


    // Print each pixel
    for pixel in &pixels {
        let color_data = pixel.color;
        let red = ((color_data >> 16) & 0xFF) as u8;
        let green = ((color_data >> 8) & 0xFF) as u8;
        let blue = (color_data & 0xFF) as u8;
        println!("Pixel - R: {}, G: {}, B: {}", red, green, blue);
    }

}
    
