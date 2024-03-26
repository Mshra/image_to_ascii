use image::{open, DynamicImage};
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
}
