use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::Path;
use std::error::Error;
use std::env;


const FILE_NAME1: &str = "layer1.tga";
const FILE_NAME2: &str = "pattern1.tga";
const PATH: &str = "/Users/j10/projects/cop3504c/project3/input/";
const SAVE_AS: &str = "test1_multiply.tga";

fn multiply(image: &mut RgbImage){
    //pixels_mut is an iterator for all the pixel rgb refrences
    for pixel in image.pixels_mut()
    {
        let [r,g,b] = pixel.0;

        let r = 255 as u8;
        let g = 0 as u8;
        let b = 0 as u8;

        *pixel = image::Rgb([r,g,b])

    }
}





fn main()-> Result<(), Box<dyn Error>>{


    let fullPath = format!("{}{}",PATH,FILE_NAME1);


//load image data using image crate
    let userImage = image::open(&Path::new(&fullPath))?;

    let (imageWidth, imageHeight) = userImage.dimensions();

    //convert image data to RGB format

    let mut rgb_data = userImage.to_rgb8();
    

    multiply(&mut rgb_data);


    rgb_data.save(format!("{}{}","../output/",SAVE_AS))?;


   Ok(()) 


}
    
