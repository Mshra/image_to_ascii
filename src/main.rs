use image::{open, DynamicImage, GenericImageView, Luma, Pixel};
use std::env;

fn main() {
    // getting image name through as a command line argument
    let args: Vec<String> = env::args().collect();

    let image_path: String = format!("images/{}", &args[1]);

    let image: DynamicImage = open(&image_path).unwrap();
    image.grayscale();

    let width: u32 = image.width();
    let height: u32 = image.height();
    let gray_ramp: String =
        String::from("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ");

    for h in 0..height {
        for w in 0..width {
            let pixel: Luma<u8> = image.get_pixel(w, h).to_luma();
            let intensity = pixel[0];
            let index = ((gray_ramp.len() - 1) as u16) * intensity as u16 / 255;

            print!("{}", &gray_ramp[index..index + 1]);
        }
        println!();
    }
}
