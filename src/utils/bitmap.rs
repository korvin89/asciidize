use image::GenericImageView;

#[derive(Debug, PartialEq, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Pixel>>,
}

pub enum ImageType {
    PNG,
}

pub fn save_bitmap_to_file(bitmap: Bitmap, filename: &str, image_type: ImageType) {
    match image_type {
        ImageType::PNG => bitmap_to_png(bitmap, filename),
    }
}

fn bitmap_to_png(bitmap: Bitmap, filename: &str) {
    let mut image_buffer = image::ImageBuffer::new(bitmap.width, bitmap.height);
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let pixel = bitmap.pixels[y as usize][x as usize].clone();
            image_buffer.put_pixel(x, y, image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
        }
    }
    image_buffer.save(filename).unwrap();
}

pub fn load_bitmap_from_file(filename: &str, image_type: ImageType) -> Bitmap {
    match image_type {
        ImageType::PNG => return png_to_bitmap(filename),
    }
}

fn png_to_bitmap(filename: &str) -> Bitmap {
    let image_buffer = image::open(filename).unwrap();
    let (width, height) = image_buffer.dimensions();
    let mut pixels = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            let pixel = image_buffer.get_pixel(x, y);
            row.push(Pixel {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            });
        }
        pixels.push(row);
    }
    return Bitmap {
        width,
        height,
        pixels,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IMAGE_FOLDER_PATH: &str = "tests/test_images";
    const TEST_IMAGE_NAME: &str = "2x2";
    const TEMP_TEST_IMAGE_FOLDER_PATH: &str = "/tmp";

    fn get_bitmap() -> Bitmap {
        let pixels = vec![
            vec![
                Pixel {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 127,
                },
                Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            ],
            vec![
                Pixel {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                Pixel {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 0,
                },
            ],
        ];
        Bitmap {
            width: 2,
            height: 2,
            pixels,
        }
    }

    fn assert_file_content(filename: &str, expected_filename: &str) {
        let expected = std::fs::read(expected_filename).unwrap();
        let actual = std::fs::read(filename).unwrap();
        assert_eq!(expected, actual);
    }

    fn get_expected_file_path(image_type: ImageType) -> std::string::String {
        let current_folder = std::path::Path::new(TEST_IMAGE_FOLDER_PATH);
        let file_path = current_folder
            .join(TEST_IMAGE_NAME)
            .with_extension(match image_type {
                ImageType::PNG => "png",
            });
        return file_path.to_string_lossy().to_string();
    }

    fn get_test_file_path(image_type: ImageType) -> std::string::String {
        let current_folder = std::path::Path::new(TEMP_TEST_IMAGE_FOLDER_PATH);
        let file_path = current_folder
            .join(TEST_IMAGE_NAME)
            .with_extension(match image_type {
                ImageType::PNG => "png",
            });
        return file_path.to_string_lossy().to_string();
    }

    #[test]
    fn test_save_bitmap_to_file() {
        let bitmap = get_bitmap();
        let filename = get_test_file_path(ImageType::PNG);
        save_bitmap_to_file(bitmap, &filename, ImageType::PNG);

        let expected_filename = get_expected_file_path(ImageType::PNG);
        assert_file_content(&filename, &expected_filename);
    }

    #[test]
    fn test_load_bitmap_from_file() {
        let expected_filename = get_expected_file_path(ImageType::PNG);
        let bitmap = load_bitmap_from_file(&expected_filename, ImageType::PNG);
        let expected_bitmap = get_bitmap();
        assert_eq!(bitmap, expected_bitmap);
    }
}
