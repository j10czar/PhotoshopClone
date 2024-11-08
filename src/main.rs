use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::Path;
use std::error::Error;
use std::env;
use itertools::izip;
use image::imageops::rotate180;
use std::fs::File;
use std::io::{Read,Write, Cursor};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};


const PATH: &str = "/Users/j10/projects/cop3504c/project3/input/";

struct Header{
    idLength: u8,
    colorMapType: u8,
    dataTypeCode: u8,
    colorMapOrigin: u16,
    colorMapLength: u16,
    colorMapDepth: u8,
    xOrigin: u16,
    yOrigin: u16,
    width: u16,
    height: u16,
    bitsPerPixel: u8,
    imageDescriptor:u8

}


//converts Image crate to bytes and writes the file at the path with the headere data from the
//og image
fn writeFileWithImage(image: &RgbImage, header_path: &str, output_path: &str)-> Result<(), Box<dyn Error>>{


    //create file that we are writing top

    let mut outputFile = File::create(output_path)?;

    //open one of the images to get header data
    let mut headerFile = File::open(header_path)?;

    //specify how long the header for all TGAs
    let mut headerRange = [0u8;18];

    headerFile.read_exact(&mut headerRange);

    //create a cursor to read the header data in the specified range
    let mut cursor = Cursor::new(&headerRange);
//use little endian becuase for tga files the smallest btye is listed with the least significant value fist

    //use the header struct we defined before to store our header data
    let idLength = cursor.read_u8()?;
    let colorMapType = cursor.read_u8()?;
    let dataTypeCode = cursor.read_u8()?;
    let colorMapOrigin = cursor.read_u16::<LittleEndian>()?;
    let colorMapLength = cursor.read_u16::<LittleEndian>()?;
    let colorMapDepth = cursor.read_u8()?;
    let xOrigin = cursor.read_u16::<LittleEndian>()?;
    let yOrigin = cursor.read_u16::<LittleEndian>()?;
    let mut width = cursor.read_u16::<LittleEndian>()?; //will be updated to image dimensions later
    let mut height = cursor.read_u16::<LittleEndian>()?;
    let bitsPerPixel = cursor.read_u8()?;
    let imageDescriptor = cursor.read_u8()?;


    //set the width and height to the imput image (just in case it differs thean the header w and h)
    let (i_width, i_height) = image.dimensions();

    width = i_width as u16;
    height = i_height as u16;

    println!("{}",width);

    println!("{}",height);

    let header = Header{
        idLength,
        colorMapType,
        dataTypeCode,
        colorMapOrigin,
        colorMapLength,
        colorMapDepth,
        xOrigin,
        yOrigin,
        width,
        height,
        bitsPerPixel,
        imageDescriptor,
    };
    
    //create empty vector to write bytes to in order to write to new file 
    let mut headerData = Vec::new();
    headerData.write_u8(header.idLength)?;
    headerData.write_u8(header.colorMapType)?;
    headerData.write_u8(header.dataTypeCode)?;
    headerData.write_u16::<LittleEndian>(header.colorMapOrigin)?;
    headerData.write_u16::<LittleEndian>(header.colorMapLength)?;
    headerData.write_u8(header.colorMapDepth)?;
    headerData.write_u16::<LittleEndian>(header.xOrigin)?;
    headerData.write_u16::<LittleEndian>(header.yOrigin)?;
    headerData.write_u16::<LittleEndian>(header.width)?;
    headerData.write_u16::<LittleEndian>(header.height)?;
    headerData.write_u8(header.bitsPerPixel)?;
    headerData.write_u8(header.imageDescriptor)?;





    //writes the data vector to the outputFile
    outputFile.write_all(&headerData)?;

    for y in (0..i_height).rev(){
        for x in 0..i_width{
            let pixel = image.get_pixel(x,y);
            let [r,g,b] = pixel.0;
            outputFile.write_all(&[b,g,r])?;
        }
    }


    Ok(())
        
    
    
    
}



fn multiply(image1: &mut RgbImage, image2: &mut RgbImage) -> RgbImage{
    //checks to see if the images are the same size
    assert_eq!(image1.dimensions(), image2.dimensions());
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w,h);


    //zips the pixels together so we can iterate through both of them
    //also iterate through the result so we can assign the respective pixel value at the right spot
    for (result_pixel, (p1, p2)) in result
    .pixels_mut() //gives the mutable refence to the pixel data
    .zip(image1.pixels().zip(image2.pixels())){
        let [r1,g1,b1] = p1.0;
        let [r2,g2,b2] = p2.0;

        //multiplies the pixels and devides by 255 to ensure they dont go out of range of the u8
        //int pixel value
        //adding 127 to fix the truncation for integer division

        let r = ((r1 as u16 * r2 as u16+127)/255) as u8;
        let g = ((g1 as u16 * g2 as u16+127)/255) as u8;
        let b = ((b1 as u16 * b2 as u16+127)/255) as u8;

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
        //adding 127 to fix rounding again...
        
        let r = (255 - (((255 - r1 as u16) * (255 - r2 as u16) + 127) / 255)) as u8;
        let g = (255 - (((255 - g1 as u16) * (255 - g2 as u16) + 127) / 255)) as u8;
        let b = (255 - (((255 - b1 as u16) * (255 - b2 as u16) + 127) / 255)) as u8;


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
        let r = if r2 <= 127 {
            let temp = (2 * r1 as u16 * r2 as u16 + 127) / 255;
            temp.clamp(0, 255) as u8
        } else {
            let temp = 255 - ((2 * (255 - r1 as u16) * (255 - r2 as u16) + 127) / 255);
            temp.clamp(0, 255) as u8
        };

        let g = if g2 <= 127 {
            let temp = (2 * g1 as u16 * g2 as u16 + 127) / 255;
            temp.clamp(0, 255) as u8
        } else {
            let temp = 255 - ((2 * (255 - g1 as u16) * (255 - g2 as u16) + 127) / 255);
            temp.clamp(0, 255) as u8
        };

        let b = if b2 <= 127 {
            let temp = (2 * b1 as u16 * b2 as u16 + 127) / 255;
            temp.clamp(0, 255) as u8
        } else {
            let temp = 255 - ((2 * (255 - b1 as u16) * (255 - b2 as u16) + 127) / 255);
            temp.clamp(0, 255) as u8
        };

        *result_pixel = image::Rgb([r, g, b]);

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




fn task9(red: &mut RgbImage, green: &mut RgbImage, blue: &mut RgbImage) -> RgbImage{
    //checks to see if the images are the same size
    let (w,h) = red.dimensions();

    let mut result = RgbImage::new(w,h);

//uses the Izip package to iterate through 3 sets of images at the same time and return respective
//pixel values
    for (result_pixel, p1, p2, p3) in izip!(
    result.pixels_mut(),
    red.pixels(),
    green.pixels(),
    blue.pixels()){


        let [r1,g1,b1] = p1.0;
        let [r2,g2,b2] = p2.0;
        let [r3,g3,b3] = p3.0;

        let r = r1;
        let g = g2;
        let b = b3;
 

        *result_pixel = image::Rgb([r,g,b])

    }

    result
}



fn bonus(image1: &mut RgbImage, image2: &mut RgbImage, image3: &mut RgbImage, image4: &mut RgbImage) -> RgbImage{
    let (w,h) = image1.dimensions();

    let mut result = RgbImage::new(w*2,h*2);
    
    //1st quadrant
    for y in 0..h{
        for x in 0..w{
            let pixel = image1.get_pixel(x,y);

            let [r,g,b] = pixel.0;

            //get the mutable refrence to the empty image pixel

            let pixel_mut = result.get_pixel_mut(x,y);

            *pixel_mut = image::Rgb([r,g,b])




        }
    }

    //2nd quadrant
    for y in 0..h{
        for x in 0..w{
            let pixel = image2.get_pixel(x,y);

            let [r,g,b] = pixel.0;

            //get the mutable refrence to the empty image pixel

            //shift refrence one picture width to the right
            let pixel_mut = result.get_pixel_mut(x+w,y);

            *pixel_mut = image::Rgb([r,g,b])




        }
    }


    //3nd quadrant
    for y in 0..h{
        for x in 0..w{
            let pixel = image3.get_pixel(x,y);

            let [r,g,b] = pixel.0;

            //get the mutable refrence to the empty image pixel

            //shift refrence one picture width down from base 0,0
            let pixel_mut = result.get_pixel_mut(x,y+h);

            *pixel_mut = image::Rgb([r,g,b])




        }
    }

    //4nd quadrant
    for y in 0..h{
        for x in 0..w{
            let pixel = image4.get_pixel(x,y);

            let [r,g,b] = pixel.0;

            //get the mutable refrence to the empty image pixel

            //shift refrence one picture width down from base 0,0 and one width to the right
            let pixel_mut = result.get_pixel_mut(x+w,y+h);

            *pixel_mut = image::Rgb([r,g,b])




        }
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


    writeFileWithImage(&output, &fullPath1, "../output/part1.tga")?;



    //task2 : subtract layer2 from car.tga -----------------
    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut fullPath2 = format!("{}{}",PATH,"layer2.tga");


//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let output = subtract(&mut userImage1, &mut userImage2);


    writeFileWithImage(&output, &fullPath1, "../output/part2.tga")?;

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


    writeFileWithImage(&output, &fullPath1, "../output/part3.tga")?;


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

    

    writeFileWithImage(&output, &fullPath1, "../output/part4.tga")?;



    //task5: combine layer1 (as top) with pattern1 using overlay--------------------------

    let mut fullPath1 = format!("{}{}",PATH,"layer1.tga");

    let mut fullPath2 = format!("{}{}",PATH,"pattern1.tga");


//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let output = overlay(&mut userImage1, &mut userImage2);



    writeFileWithImage(&output, &fullPath1, "../output/part5.tga")?;

    //task 6: load car.tga and add 200 to the green channel----------------------
    
    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let output = task6(&mut userImage1);

    writeFileWithImage(&output, &fullPath1, "../output/part6.tga")?;




    //task 7: load car and multiply red by 4 and negate all blue --------
    
    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let output = task7(&mut userImage1);

    writeFileWithImage(&output, &fullPath1, "../output/part7.tga")?;


    //task 8:load car and seperate the r g and b values to different files ------


    let mut fullPath1 = format!("{}{}",PATH,"car.tga");
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();
    
    let outputr = task8r(&mut userImage1);


    let outputg = task8g(&mut userImage1);


    let outputb = task8b(&mut userImage1);

    writeFileWithImage(&outputr, &fullPath1, "../output/part8_r.tga")?;


    writeFileWithImage(&outputg, &fullPath1, "../output/part8_g.tga")?;


    writeFileWithImage(&outputb, &fullPath1, "../output/part8_b.tga")?;


    //task 9: merge the seperated images and make them into 1-------------


    let mut fullPath1 = format!("{}{}",PATH,"layer_red.tga");

    let mut fullPath2 = format!("{}{}",PATH,"layer_green.tga");

    let mut fullPath3 = format!("{}{}",PATH,"layer_blue.tga");

//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let mut userImage3 = image::open(&Path::new(&fullPath3))?.to_rgb8();
    


    let output = task9(&mut userImage1, &mut userImage2, &mut userImage3);

    writeFileWithImage(&output, &fullPath1, "../output/part9.tga")?;


    //task 10: rotate text2 180----------
    
    //this can be accomplished very easily (huge shoutout to rust image module)
    

    let mut fullPath1 = format!("{}{}",PATH,"text2.tga");

    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let output = rotate180(&userImage1);

    writeFileWithImage(&output, &fullPath1, "../output/part10.tga")?;

    //lol


    //extra credit create a car circles pattern and text at each quadrant of the screen-------------------


    let mut fullPath1 = format!("{}{}",PATH,"car.tga");

    let mut fullPath2 = format!("{}{}",PATH,"circles.tga");

    let mut fullPath3 = format!("{}{}",PATH,"text.tga");

    let mut fullPath4 = format!("{}{}",PATH,"pattern1.tga");

//load image data using image crate
    let mut userImage1 = image::open(&Path::new(&fullPath1))?.to_rgb8();

    let mut userImage2 = image::open(&Path::new(&fullPath2))?.to_rgb8();

    let mut userImage3 = image::open(&Path::new(&fullPath3))?.to_rgb8();

    let mut userImage4 = image::open(&Path::new(&fullPath4))?.to_rgb8();



    let output = bonus(&mut userImage1, &mut userImage2, &mut userImage3, &mut userImage4);

    writeFileWithImage(&output, &fullPath1, "../output/extracredit.tga")?;

   Ok(()) 


}
    
