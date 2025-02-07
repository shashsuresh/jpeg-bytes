use std::{fs::{create_dir, File}, io::Write};

use image::{load_from_memory, DynamicImage, ImageReader};

fn main() {

    //TODO check if it exists and if it does, don't create
    create_dir("./output").unwrap();
    // TODO recursively convert images

    let image = ImageReader::open("./images/image.jpg").unwrap().decode().unwrap();

    let image = image.resize(128, 128, image::imageops::FilterType::Triangle);

    let image_bytes = image.into_bytes();

    let mut file = File::create("./output/image.h").unwrap();

    file.write_fmt(format_args!("const unsigned char image[] = {{")).unwrap();
    for byte in image_bytes {
        file.write_fmt(format_args!("{},",byte)).unwrap();
    }
    file.write_fmt(format_args!("}};")).unwrap();
}
