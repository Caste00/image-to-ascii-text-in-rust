use crate::img_to_ascii::{img_to_ascii, save_ascii_img, scale_img};
use std::io::{stdin, Write};

mod img_to_ascii;
mod pixel_struct;

fn main() {
    let mut path = String::new();
    print!("Enter the path to the image: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    let mut scale = String::new();
    print!("Enter the scale of the image: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut scale).unwrap();
    let scale = scale.trim().parse::<u32>().unwrap();

    let mut contrast = String::new();
    print!("Enter the contrast of the image: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut contrast).unwrap();
    let contrast = contrast.trim().parse::<f32>().unwrap();

    let pixel = scale_img(path, scale, contrast);
    let pixel_ascii = img_to_ascii(pixel);
    save_ascii_img(&pixel_ascii);
}