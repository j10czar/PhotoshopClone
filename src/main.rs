use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::Path;
use std::error::Error;
use std::env;


const FILE_NAME1: &str = "layer1.tga";
const FILE_NAME2: &str = "pattern2.tga";
const FILE_NAME3: &str = "text.tga";
const PATH: &str = "/Users/j10/projects/cop3504c/project3/input/";
const SAVE_AS: &str = "task3_multiply_screen.tga";

fn multiply(image1: &mut RgbImage, image2: &mut RgbImage) -> RgbImage{
    //checks to see if the images are the same size
    assert_eq!(image1.dimensions(), image2.dimensions());
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    //zips the pixels together so we can iterate through both of them
    //also iterate through the result so we can assign the respective pixel value at the right spot
    for (result_pixel, (p1, p2)) in result
    .pixels_mut()
    .zip(image1.pixels().zip(image2.pixels())){
        let [r1,g1,b1] = p1.0;
        let [r2,g2,b2] = p2.0;

        //multiplies the pixels and devides by 255 to ensure they dont go out of range of the u8
        //int pixel value

        let r = (r1 as u16 * r2 as u16/255) as u8;
        let g = (g1 as u16 * g2 as u16/255) as u8;
        let b = (b1 as u16 * b2 as u16/255) as u8;

        *result_pixel = image::Rgb([r,g,b])

    }

    result
}

//subtracts image2 from image1
fn subtract(image1: &mut RgbImage, image2: &mut RgbImage) -> RgbImage{
    //checks to see if the images are the same size
    assert_eq!(image1.dimensions(), image2.dimensions());
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    //zips the pixels together so we can iterate through both of them
    //also iterate through the result so we can assign the respective pixel value at the right spot
    for (result_pixel, (p1, p2)) in result
    .pixels_mut()
    .zip(image1.pixels().zip(image2.pixels())){
        let [r1,g1,b1] = p1.0;
        let [r2,g2,b2] = p2.0;

        
        //uses the built in rust saturating sub function which automatically floors and ceilings
        //the values to prevent overflow
        let r = r1.saturating_sub(r2);
        let g = g1.saturating_sub(g2);
        let b = b1.saturating_sub(b2);

        *result_pixel = image::Rgb([r,g,b])

    }

    result
}


fn screen(image1: &mut RgbImage, image2: &mut RgbImage) -> RgbImage{
    //checks to see if the images are the same size
    assert_eq!(image1.dimensions(), image2.dimensions());
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    //zips the pixels together so we can iterate through both of them
    //also iterate through the result so we can assign the respective pixel value at the right spot
    for (result_pixel, (p1, p2)) in result
    .pixels_mut()
    .zip(image1.pixels().zip(image2.pixels())){
        let [r1,g1,b1] = p1.0;
        let [r2,g2,b2] = p2.0;

        //screen formula 1-(1-A)*(1-B)
        //remember 1 is a representation of 255
        
        let r = (255-(((255-r1 as u16)*(255-r2 as u16))/255)) as u8;

        let g = (255-(((255-g1 as u16)*(255-g2 as u16))/255)) as u8;

        let b = (255-(((255-b1 as u16)*(255-b2 as u16))/255)) as u8;



        *result_pixel = image::Rgb([r,g,b])

    }

    result
}


fn main()-> Result<(), Box<dyn Error>>{


    let fullPath1 = format!("{}{}",PATH,FILE_NAME1);

    let fullPath2 = format!("{}{}",PATH,FILE_NAME2);

    let fullPath3 = format!("{}{}",PATH,FILE_NAME3);


//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let mut userImage3 = image::open(&Path::new(&fullPath3))?.to_rgb8();
    

    let mut mid = multiply(&mut userImage1,&mut userImage2);

    let output = screen(&mut mid, &mut userImage3);

    output.save(format!("{}{}","../output/",SAVE_AS))?;


   Ok(()) 


}
    
