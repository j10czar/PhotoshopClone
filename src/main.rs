use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::Path;
use std::error::Error;
use std::env;


const PATH: &str = "/Users/j10/projects/cop3504c/project3/input/";


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
    

    //task 1: multiply layer1 and pattern1


    let mut fullPath1 = format!("{}{}",PATH,"layer1.tga");

    let mut fullPath2 = format!("{}{}",PATH,"pattern1.tga");


//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let output = multiply(&mut userImage1, &mut userImage2);

    output.save("../output/part1.tga")?;




    //task2 : subtract layer2 from car.tga -----------------
    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut fullPath2 = format!("{}{}",PATH,"layer2.tga");


//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let output = subtract(&mut userImage1, &mut userImage2);

    output.save("../output/part2.tga")?;


     //task3: multiply layer1 and pattern2 then screen it with text -----------------
    let mut fullPath1 = format!("{}{}",PATH,"layer1.tga");

    let mut fullPath2 = format!("{}{}",PATH,"pattern2.tga");

    let mut fullPath3 = format!("{}{}",PATH,"text.tga");

//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let mut userImage3 = image::open(&Path::new(&fullPath3))?.to_rgb8();


    let mut mid = multiply(&mut userImage1, &mut userImage2);

    let output = screen(&mut userImage3, &mut mid);

    output.save("../output/part3.tga")?;

   Ok(()) 


}
    
