use image::{open, DynamicImage, GenericImageView, Luma, Pixel};
use std::collections::HashMap;
use std::env;

fn max_dimensions(width: u32, height: u32) -> (u32, u32) {
    const MAX_WIDTH: u32 = 80;
    const MAX_HEIGHT: u32 = 50;

    if height > MAX_HEIGHT {
        (width * MAX_HEIGHT / height, MAX_HEIGHT)
    } else if width > MAX_WIDTH {
        (height * MAX_WIDTH / width, MAX_WIDTH)
    } else {
        (width, height)
    }
}

fn main() {
    // getting image name through as a command line argument
    let args: Vec<String> = env::args().collect();

    let image_path: String = format!("images/{}", &args[1]);

    let image: DynamicImage = open(&image_path).unwrap();
    image.grayscale();

    let mut gray_ramp = HashMap::new();
    for (k, v) in "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. "
        .chars()
        .enumerate()
    {
        gray_ramp.insert(k, v);
    }

    let (width, height) = max_dimensions(image.width(), image.height());

    for h in 0..height {
        for w in 0..width {
            let pixel: Luma<u8> = image.get_pixel(w, h).to_luma();
            let intensity = pixel[0];
            let index: usize = (((gray_ramp.len() - 1) as u16) * intensity as u16 / 255) as usize;

            print!("{}", gray_ramp.get(&index).unwrap());
        }
        println!();
    }
}
