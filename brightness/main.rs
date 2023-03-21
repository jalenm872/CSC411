use csc411_image::{Read, GrayImage};
use std::env;

fn main() {
    let input = env::args().nth(1);
    assert!(env::args().len() == 2);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let h = img.height as u32;
    let w = img.width as u32;
    let denom = img.denominator as f32;
    let mut total = 0 as u32;
    //For each pixel in the image, add it to the total
    for pixel in img.pixels {
        total += pixel.value as u32;
    }
    let new_denom = (h*w) as f32; 
    let answer = total as f32/new_denom;
    //Print the average brightness
    println!("{:.3}", answer/denom as f32);
} 