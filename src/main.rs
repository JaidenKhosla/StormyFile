use image::{self, DynamicImage, GenericImageView, RgbImage};
use std::fs::{self, File};
use std::{path::Path};

use std::io::BufReader;


fn resize(data_length: usize) -> image::RgbImage
{
    
    let input = std::env::current_dir().unwrap().join("YEET.png");
    let img: image::DynamicImage = image::open(input).unwrap();
    
    let nearest_perfect = (data_length as f64/3.0).sqrt().ceil() as u32;

    img.resize(nearest_perfect, nearest_perfect, image::imageops::FilterType::Nearest).to_rgb8()
}

fn encode(bytes: &Vec<u8>) -> RgbImage
{
    let mut img = resize(bytes.len());

    let header_signature = format!("{}\0", bytes.len());


    for idx in 0..bytes.len()
    {
        let pixel_r = (idx/3) as u32 / img.height();
        let pixel_c = (idx/3) as u32 % img.width();

        let mut rgb_color = img.get_pixel(pixel_c  ,pixel_r).0;


        rgb_color[idx%3] ^= bytes[idx];

        img.put_pixel(pixel_c, pixel_r,image::Rgb(rgb_color));
    }
    
    img
}

fn decode(path: &Path) -> Vec<u8>
{
    let img = image::open(path).unwrap();

    let size = (img.width() * img.height() *3) as usize;

    let resized_reference = resize(size);

    let mut data: Vec<u8> = vec![];

    for idx in 0..size
    {
        let x = (idx/3) as u32%img.width();
        let y = (idx/3) as u32/img.height();

        let data_pixel = img.get_pixel(x, y);
        let referenced_pixel = resized_reference.get_pixel(x, y);

        data.push(data_pixel[idx%3] ^ referenced_pixel[idx%3]);
    }


    data
}

fn main() {
    
    let DATA = "Hello World!".bytes().collect::<Vec<u8>>();

    let test = fs::read("output2.png").unwrap();

    println!("{:?}", test);


    println!("{:?}", &DATA);

    let img = encode(&DATA);


    // img.save("output2");
    let bytes = decode(&std::env::current_dir().unwrap().join("output2.png"));

    println!("{:?}", &bytes);
    println!("{}", String::from_utf8_lossy(&bytes));
}
