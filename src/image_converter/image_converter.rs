use crate::utils::bitmap::{Bitmap, ImageType, load_bitmap_from_file, save_bitmap_to_file};

// pub fn bitmap_to_grayscale (filename: &str, image_type: &ImageType) {
//     let mut bitmap = load_bitmap_from_file(filename, image_type);
//     for y in 0..bitmap.height {
//         for x in 0..bitmap.width {
//             let mut pixel = bitmap.pixels[y as usize][x as usize].clone();
//             let value = (((pixel.r as u16 + pixel.g as u16 + pixel.b as u16) as u16) / 3) as u8;
//             pixel.r = value;
//             pixel.g = value;
//             pixel.b = value;
//             bitmap.pixels[y as usize][x as usize] = pixel;
//         }
//     }
//     save_bitmap_to_file(&bitmap, "/Users/alaev89/Desktop/pets/asciidize/src/test2.png", image_type);
// }

pub fn bitmap_to_grayscale (bitmap: &Bitmap) -> Bitmap {
    let mut bitmap_grayscale = bitmap.clone();
    for y in 0..bitmap_grayscale.height {
        for x in 0..bitmap_grayscale.width {
            let mut pixel = bitmap_grayscale.pixels[y as usize][x as usize].clone();
            let value = (((pixel.r as u16 + pixel.g as u16 + pixel.b as u16) as u16) / 3) as u8;
            pixel.r = value;
            pixel.g = value;
            pixel.b = value;
            bitmap_grayscale.pixels[y as usize][x as usize] = pixel;
        }
    }
    return bitmap_grayscale;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_to_grayscale() {
        // let expected_filename = get_expected_file_path(&ImageType::PNG);
        // let bitmap = load_bitmap_from_file(&expected_filename, &ImageType::PNG);
        // let expected_bitmap = get_bitmap();
        // assert_eq!(bitmap, expected_bitmap);
        
    }
}
