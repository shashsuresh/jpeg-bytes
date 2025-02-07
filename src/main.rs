use std::{fs::{create_dir, read_dir, File, ReadDir}, io::Write, path::Path};

use image::{imageops::FilterType::Triangle, ImageReader};

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

            let image = image.resize_exact(80, 80, Triangle);
        
            let image_bytes = image.into_bytes();

            file.write_fmt(format_args!("const unsigned char {}[] = {{", entry.to_str().unwrap().strip_suffix(".jpg").unwrap().strip_prefix("./images/").unwrap())).unwrap();
            let mut elem_ctr = 0;
            for byte in image_bytes {
                file.write_fmt(format_args!("{}, ",byte)).unwrap();
                elem_ctr+=1;
                if elem_ctr == 45 {
                    file.write_fmt(format_args!("\n")).unwrap();
                    elem_ctr = 0;
                }
            }
            file.write_fmt(format_args!("}};\n")).unwrap();
        }
    }
}
