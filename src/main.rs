use image::{self, DynamicImage, RgbImage};
use std::{path::Path};


fn resize(data_length: usize) -> image::RgbImage
{
    
    let input = std::env::current_dir().unwrap().join("YEET.png");
    let img: image::DynamicImage = image::open(input).unwrap();
    
    let nearest_perfect = (data_length as f64).sqrt().ceil() as u32;

    img.resize(nearest_perfect, nearest_perfect, image::imageops::FilterType::Nearest).to_rgb8()
}

fn compress(bytes: &Vec<u8>) -> RgbImage
{
    let mut img = resize(bytes.len());

    for idx in 0..bytes.len()
    {
        let r = idx as u32 / img.height();
        let c = idx as u32 % img.width();

        let mut rgb_color = img.get_pixel(c, r).0;

        for channel in 0..3
        {
            rgb_color[channel] ^= bytes[idx];
        }

        img.put_pixel(c, r,image::Rgb(rgb_color));
    }
    
    img
}
fn main() {
    
    let DATA = "Hello World!".bytes().collect::<Vec<u8>>();


    let img = compress(&DATA);

    img.save("output2.png");
}
