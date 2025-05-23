use crate::pixel_struct::Pixel;
use image::{imageops::FilterType, open, DynamicImage, GenericImageView};
use std::io::Write;

pub fn scale_img(path: &str, scale: u32, constrast_factor: f32) -> Vec<Pixel> {
    let img = open(path).expect("Failed to load image");
    let (width, height) = img.dimensions();

    let img_gray = img.grayscale();
    let img_constrasted = increase_contrast(&img_gray, constrast_factor);
    let img_resize = img_constrasted.resize(width / scale, height / scale, FilterType::Nearest);

    let mut pixels: Vec<Pixel> = Vec::new();

    for (x, y, color) in img_resize.pixels() {
        pixels.push(Pixel::new(x, y, color));
    }

    pixels
}

fn increase_contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    let gray_image = img.to_luma8();
    let (width, height) = gray_image.dimensions();

    let contrasted = image::GrayImage::from_fn(width, height, |x, y| {
        let pixel = gray_image.get_pixel(x, y)[0];
        let centered = pixel as f32 - 128.0;
        let adjusted = (centered * factor).clamp(-128.0, 127.0);
        let new_pixel = (adjusted + 128.0).round().clamp(0.0, 255.0) as u8;
        image::Luma([new_pixel])
    });

    DynamicImage::ImageLuma8(contrasted)
}

fn twice<F: FnMut()>(mut f: F) {
    for _ in 0..4 {
        f();
    }
}

pub fn img_to_ascii(img: Vec<Pixel>) -> Vec<char> {
    let pixel_char = ('.', ',', '`', '-', '~', '+', '*', '!', '?', '%', '#', '@', '$', '&', '█');
    let mut img_char: Vec<char> = Vec::new();
    for ele in img {
        if ele.get_position().0 == 0 {
            img_char.push('\n');
        }
        match ele.get_color()[0] {
            0..17 => twice(|| img_char.push(pixel_char.0)),
            17..34 => twice(|| img_char.push(pixel_char.1)),
            34..51 => twice(|| img_char.push(pixel_char.2)),
            51..68 => twice(|| img_char.push(pixel_char.3)),
            68..85 => twice(|| img_char.push(pixel_char.4)),
            85..102 => twice(|| img_char.push(pixel_char.5)),
            102..119 => twice(|| img_char.push(pixel_char.6)),
            119..136 => twice(|| img_char.push(pixel_char.7)),
            136..153 => twice(|| img_char.push(pixel_char.8)),
            153..170 => twice(|| img_char.push(pixel_char.9)),
            170..187 => twice(|| img_char.push(pixel_char.10)),
            187..204 => twice(|| img_char.push(pixel_char.11)),
            204..221 => twice(|| img_char.push(pixel_char.12)),
            221..238 => twice(|| img_char.push(pixel_char.13)),
            _ => twice(|| img_char.push(pixel_char.14))
        }
    }

    img_char
}

pub fn save_ascii_img(img_char: &Vec<char>) {
    let mut file = std::fs::File::create("image_ascii.txt").expect("error to create the file");
    for ele in img_char {
        file.write_all(ele.to_string().as_bytes()).expect("error to write the file");
    }
    file.flush().expect("error to flush the file");
    println!("Image created");
}
