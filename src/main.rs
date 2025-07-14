use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};
use image::ImageReader;
use std::io::stdin;
use glob::glob;
use rgb::RGBA;
use ravif::*;

const FILE_MASK: [&str; 4] = ["png", "jpg", "jpeg", "webp"];
const DEFAULT_QUALITY: u8 = 85;
const DEFAULT_SPEED: u8 = 8;

fn main() {
    let options = main_menu();
    let images = retrieve_images(&options[0]);
    let mut exit = String::new();
    create_output_folder();
    for (index, i) in images.iter().enumerate() {
        println!("Compressing image: {} of {}", index + 1, images.len());
        compress_image(&i, options[1].parse::<u8>().expect("Invalid quality value"), options[2].parse::<u8>().expect("Invalid speed value"));
    }
    println!("Process completed. Check the 'output' folder.");
    println!("Press Enter to exit...");
    stdin().read_line(&mut exit).unwrap();
}

fn main_menu() -> Vec<String> {
    let mut path = String::new();
    let mut quality = String::new();
    let mut speed = String::new();

    println!("Welcome to the Image Compression Tool!");

    println!("Please enter the path of the image folder: ");
    stdin().read_line(&mut path).expect("Failed to read input");

    println!("Please enter the desired quality (1-100). Default: {DEFAULT_QUALITY}");
    stdin().read_line(&mut quality).expect("Failed to read input");

    println!("Please enter the desired speed (1-10). Default: {DEFAULT_SPEED}");
    stdin().read_line(&mut speed).expect("Failed to read input");

    path = path.trim().to_string();
    let quality = if quality.trim().is_empty() { DEFAULT_QUALITY.to_string() } else { quality.trim().to_string() };
    let speed = if speed.trim().is_empty() { DEFAULT_SPEED.to_string() } else { speed.trim().to_string() };

    vec![path, quality, speed]
}

fn retrieve_images(path: &str) -> Vec<PathBuf> {
    let mut images_path = Vec::new();
    let clean_path = Path::new(path);

    for ext in FILE_MASK {
        let pattern = clean_path.join(format!("*.{}", ext));
        let pattern_str = pattern.to_string_lossy();
        for entry in glob(&pattern_str).expect("Error reading pattern") {
            match entry {
                Ok(path) => images_path.push(path),
                Err(e) => eprintln!("Error reading files: {}", e),
            }
        }
    }
    println!("{} images found.", images_path.len());
    images_path
}

fn create_output_folder() {
    let output_path = Path::new("output");
    create_dir_all(&output_path).expect("Failed to create output directory");
}

fn compress_image(image_path: &Path, quality: u8, speed: u8) {
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let pixels = rgba.into_vec();

    let rgba_pixels: Vec<RGBA<u8>> = pixels
        .chunks_exact(4)
        .map(|chunk| RGBA::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect();

    let result = Encoder::new()
        .with_quality(quality as f32)
        .with_speed(speed as u8)
        .encode_rgba(Img::new(&rgba_pixels, width as usize, height as usize))
        .expect("Error encoding image");

    let file_stem = image_path.file_stem().unwrap().to_string_lossy();
    let output_path = Path::new("output").join(format!("{}.avif", file_stem));

    write(&output_path, &result.avif_file).expect("Failed to write AVIF file");
}