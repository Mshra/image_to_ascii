use image::{open, DynamicImage, GenericImageView, Luma, Pixel};
use std::collections::HashMap;
use std::env;

fn main() {
    // getting image name through as a command line argument
    let args: Vec<String> = env::args().collect();

    let image_path: String = format!("images/{}", &args[1]);

    let image: DynamicImage = open(&image_path).unwrap();
    let image = image.grayscale();

    let mut gray_ramp = HashMap::new();
    for (k, v) in "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. "
        .chars()
        .enumerate()
    {
        gray_ramp.insert(k, v);
    }

    for h in 0..image.height() {
        for w in 0..image.width() {
            let pixel: Luma<u8> = image.get_pixel(w, h).to_luma();
            let intensity = pixel[0];
            let index: usize = (((gray_ramp.len() - 1) as u16) * intensity as u16 / 255) as usize;

            print!("{}", gray_ramp.get(&index).unwrap());
        }
        println!();
    }
}
