use std::{fs::{create_dir, read_dir, File, ReadDir}, io::Write, path::Path};

use image::ImageReader;

fn main() {

    let output_dir = Path::new("./output");
    if !output_dir.exists() {
        create_dir(output_dir).unwrap()
    }

    let mut file = File::create("./output/image.h").unwrap();

    let image_dir = Path::new("./images");
    if image_dir.is_dir() {
        for image_entry in read_dir(image_dir).unwrap() {
            let entry = image_entry.unwrap().path();
            let image = ImageReader::open(entry.clone()).unwrap().decode().unwrap();

            let image = image.resize(128, 128, image::imageops::FilterType::Triangle);
        
            let image_bytes = image.into_bytes();

            file.write_fmt(format_args!("const unsigned char {} = {{", entry.to_str().unwrap().strip_suffix(".jpg").unwrap().strip_prefix("./images/").unwrap())).unwrap();
            for byte in image_bytes {
                file.write_fmt(format_args!("{},",byte)).unwrap();
            }
            file.write_fmt(format_args!("}};\n")).unwrap();
        }
    }
}
