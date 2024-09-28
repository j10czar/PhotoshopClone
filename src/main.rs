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




fn overlay(image1: &mut RgbImage, image2: &mut RgbImage) -> RgbImage{
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

        //if B<=0.5: C=2*A*B   
        //if B>0.5 C=1-2*(1-A)*(1-B)
        
        let r = if r2 <= 127{
            let temp = (2*r1 as u32 *r2 as u32)/255;
            //no semicolon to allow to set the value of r
            temp.clamp(0,255) as u8
        }
        else{
            let mut temp = 255-2*(((255-r1 as u16)*(255-r2 as u16))/255);
            temp.clamp(0,255) as u8

        };

        let g = if g2 <= 127{
            let temp = (2*g1 as u32 *g2 as u32)/255;
            temp.clamp(0,255) as u8
        }
        else{
            let mut temp = 255-2*(((255-g1 as u16)*(255-g2 as u16))/255);
            temp.clamp(0,255) as u8


        };

        let b = if b2 <= 127{
            let temp = (2*b1 as u32 *b2 as u32)/255;
            temp.clamp(0,255) as u8
        }
        else{
            let mut temp = 255-2*(((255-b1 as u16)*(255-b2 as u16))/255);
            temp.clamp(0,255) as u8

        };



        *result_pixel = image::Rgb([r,g,b])

    }

    result
}



fn task6(image1: &mut RgbImage) -> RgbImage{
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    for (result_pixel, p) in result
    .pixels_mut().zip(image1.pixels()){
        let [pr,pg,pb] = p.0;

        //adds 200 to each green value
        
        let r = pr;

        let temp = pg as u16 + 200;

        let g = temp.clamp(0,255) as u8;

        let b = pb;


        *result_pixel = image::Rgb([r,g,b])

    }

    result
}


fn task7(image1: &mut RgbImage) -> RgbImage{
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    for (result_pixel, p) in result
    .pixels_mut().zip(image1.pixels()){
        let [pr,pg,pb] = p.0;

       //multipies red by 4 and negates blue 
       


        let temp = 4*pr as u16;
        let r = temp.clamp(0,255) as u8;

        let g = pg;

        let b = pb*0 as u8;


        *result_pixel = image::Rgb([r,g,b])

    }

    result
}



fn task8r(image1: &mut RgbImage) -> RgbImage{
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    for (result_pixel, p) in result
    .pixels_mut().zip(image1.pixels()){
        let [pr,pg,pb] = p.0;

      //only gets r from image 


        let r = pr;

        let g = pr;

        let b = pr;


        *result_pixel = image::Rgb([r,g,b])

    }

    result
}




fn task8g(image1: &mut RgbImage) -> RgbImage{
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    for (result_pixel, p) in result
    .pixels_mut().zip(image1.pixels()){
        let [pr,pg,pb] = p.0;

      //only gets g from image 


        let g = pg;

        let r = pg;

        let b = pg;


        *result_pixel = image::Rgb([r,g,b])

    }

    result
}


fn task8b(image1: &mut RgbImage) -> RgbImage{
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    for (result_pixel, p) in result
    .pixels_mut().zip(image1.pixels()){
        let [pr,pg,pb] = p.0;

      //only gets b from image 


        let b = pb;

        let g = pb;

        let r = pb;


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



    //task 4: multiply layer2 and circles then load pattern 2 and subtract the multiplication from
    //pattern 2 -------------
    

    
    let mut fullPath1 = format!("{}{}",PATH,"layer2.tga");

    let mut fullPath2 = format!("{}{}",PATH,"circles.tga");

    let mut fullPath3 = format!("{}{}",PATH,"pattern2.tga");

//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let mut userImage3 = image::open(&Path::new(&fullPath3))?.to_rgb8();


    let mut mid = multiply(&mut userImage1, &mut userImage2);


    let output = subtract(&mut mid,&mut userImage3);

    

    output.save("../output/part4.tga")?;


    //task5: combine layer1 (as top) with pattern1 using overlay--------------------------

    let mut fullPath1 = format!("{}{}",PATH,"layer1.tga");

    let mut fullPath2 = format!("{}{}",PATH,"pattern1.tga");


//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let output = overlay(&mut userImage1, &mut userImage2);


    output.save("../output/part5.tga")?;


    //task 6: load car.tga and add 200 to the green channel----------------------
    
    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let output = task6(&mut userImage1);

    output.save("../output/part6.tga")?;

    //task 7: load car and multiply red by 4 and negate all blue --------
    
    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let output = task7(&mut userImage1);

    output.save("../output/part7.tga")?;


    //task 8:load car and seperate the r g and b values to different files ------


    let mut fullPath1 = format!("{}{}",PATH,"car.tga");
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();
    
    let outputr = task8r(&mut userImage1);


    let outputg = task8g(&mut userImage1);


    let outputb = task8b(&mut userImage1);

    outputr.save("../output/part8_r.tga")?;


    outputg.save("../output/part8_g.tga")?;


    outputb.save("../output/part8_b.tga")?;



   Ok(()) 


}
    
