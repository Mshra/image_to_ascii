use image::{open, DynamicImage};
use std::env;

fn open_image(image_name: &str) -> DynamicImage {
    let image_path: String = format!("images/{}", image_name);
    let image: DynamicImage = open(image_path).unwrap();
    image
}

fn main() {
    // getting image name through command line arguments
    let args: Vec<String> = env::args().collect();
    let image = open_image(&args[1])
        .grayscale()
        .save("images/amongus_grayscale.png")
        .unwrap();

    // indexing in a string data type
    // let hello: String = String::from("Hello, world");
    // println!("5th element in the list is {:?}", hello.get(5..6));
}
