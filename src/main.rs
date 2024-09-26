use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use tinytga::{Bpp, Compression, DataType, ImageOrigin, RawPixel, RawTga, TgaHeader};



//converts img data to a readable vector filled with tuples in the following format:
//(r,g,b)
fn toPixelVector(data: &RawTga<'_>){

   //allocate a new pixel vector in memory for rgb tuples 
    let mut pixel_vector = Vec::new();

    let pixels: Vec<_> = data.pixels().collect();


    for pixel in &pixels {
        let color_data = pixel.color;
        //perform bitwise operations to get the real pixel values bc tinytga gives differnt values
        //with differnet channels
        let red = ((color_data >> 16) & 0xFF) as u8;
        let green = ((color_data >> 8) & 0xFF) as u8;
        let blue = (color_data & 0xFF) as u8;

        pixel_vector.push((red,green,blue))
    }


    pixel_vector

}



fn main(){

    // Include an image from a local path as bytes.
    let data = include_bytes!("../input/car.tga");

    // Create a TGA instance from a byte slice.
    let img = RawTga::from_slice(data).unwrap();




    println!("{}","hello")


}
    
