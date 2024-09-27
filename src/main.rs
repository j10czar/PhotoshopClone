use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::Path;
use std::error::Error;
use std::env;


const FILE_NAME: &str = "car.tga";
const PATH: &str = "/Users/j10/projects/cop3504c/project3/input/";



fn main()-> Result<(), Box<dyn Error>>{


    let fullPath = format!("{}{}",PATH,FILE_NAME);


//load image data using image crate
    let userImage = image::open(&Path::new(&fullPath))?;

    let (imageWidth, imageHeight) = userImage.dimensions();

    //convert image data to RGB format

    let mut rgb_data = userImage.to_rgb8();

    rgb_data.save(&format!("{}{}","../output/edited_",FILE_NAME))?;


   Ok(()) 


}
    
