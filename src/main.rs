use image::DynamicImage;
use image::ImageReader;
use std::{fs, path};
use glob::glob;
use rgb::RGBA;
use ravif::*;

const  FILE_MASK: [&str; 4] = ["png", "jpg", "jpeg", "webp"];
const SPEED: u8 = 5;      //1-10, slowest to fastest

fn main() {
    let options = main_menu();
    retrieve_images(&options[0]);
/* 

    let img = ImageReader::open("wall52.png").expect("Failed to open image");
    let dyn_img = img.decode().expect("Failed to decode image");
    let rgba = dyn_img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let pixels = rgba.into_vec();

    let rgba_pixels: Vec<RGBA<u8>> = pixels
        .chunks_exact(4)
        .map(|chunk| RGBA::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect();

    let result = Encoder::new()
        .with_quality(80.0)
        .with_speed(SPEED)
        .encode_rgba(Img::new(&rgba_pixels, width as usize, height as usize))
        .expect("Error");

    fs::write("image.avif", &result.avif_file).expect("Failed to write AVIF file"); */
}

fn main_menu() -> Vec<String> {
    let mut path = String::new();
    let mut quality = String::new();

    println!("Welcome to the Image Compression Tool!");

    println!("Please enter the path of the image folder: ");
    std::io::stdin().read_line(&mut path).expect("Failed to read input");
    
    println!("Please enter the desired quality (1-100): ");
    std::io::stdin().read_line(&mut quality).expect("Failed to read input");

    path = path.trim().to_string();
    quality = quality.trim().to_string();

    return vec![path, quality];
}

fn retrieve_images(path: &str) -> Vec<DynamicImage> {
    let mut images_path = Vec::new();
    let mut images_array = Vec::new();
    let clean_path = path.replace("\\", "/").trim_end_matches('/').to_string();

    for ext in FILE_MASK {
        let pattern = format!("{}/*.{}", clean_path, ext);
        for entry in glob(&pattern).expect("Error reading pattern") {
            match entry {
                Ok(path) => images_path.push(path.to_string_lossy().to_string()),
                Err(e) => eprintln!("Error reading files: {}", e),
            }
        }
    }
    for image in images_path {
        let img = ImageReader::open(image)
            .expect("Failed to open image")
            .decode()
            .expect("Failed to decode image");
        images_array.push(img);
    }
    images_array
}