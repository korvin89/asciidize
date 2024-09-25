use image::GenericImageView;

#[derive(Debug, PartialEq, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    /// # Panics
    ///
    /// Will panic if pixel has incorrect rgb values
    pub fn to_grayscale(&mut self) {
        let value =
            u8::try_from((u16::from(self.r) + u16::from(self.g) + u16::from(self.b)) / 3).unwrap();
        self.r = value;
        self.g = value;
        self.b = value;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Bitmap {
    pub fn to_grayscale(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[y as usize][x as usize].to_grayscale();
            }
        }
    }
}

pub enum ImageType {
    PNG,
}

pub fn save_to_file(bitmap: &Bitmap, filename: &str, image_type: &ImageType) {
    match image_type {
        ImageType::PNG => save_to_png_file(bitmap, filename),
    }
}

fn save_to_png_file(bitmap: &Bitmap, filename: &str) {
    let mut image_buffer = image::ImageBuffer::new(bitmap.width, bitmap.height);
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let pixel = bitmap.pixels[y as usize][x as usize].clone();
            image_buffer.put_pixel(x, y, image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
        }
    }
    image_buffer.save(filename).unwrap();
}

#[must_use]
pub fn load_from_file(filename: &str, image_type: &ImageType) -> Bitmap {
    match image_type {
        ImageType::PNG => return load_from_png_file(filename),
    }
}

fn load_from_png_file(filename: &str) -> Bitmap {
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
    use crate::utils::test;

    use super::*;

    const IMAGE_2X2: &str = "2x2";
    const IMAGE_FROG_COLOR: &str = "frog_32x32_color";
    const IMAGE_FROG_GRAY: &str = "frog_32x32_gray";

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

    #[test]
    fn test_save_to_file() {
        let bitmap = get_bitmap();
        let filename = test::get_test_file_path(&ImageType::PNG, IMAGE_2X2);
        save_to_file(&bitmap, &filename, &ImageType::PNG);

        let expected_filename = test::get_expected_file_path(&ImageType::PNG, IMAGE_2X2);
        assert_file_content(&filename, &expected_filename);

        std::fs::remove_file(&filename).unwrap();
    }

    #[test]
    fn test_load_from_file() {
        let expected_filename = test::get_expected_file_path(&ImageType::PNG, IMAGE_2X2);
        let bitmap = load_from_file(&expected_filename, &ImageType::PNG);
        let expected_bitmap = get_bitmap();
        assert_eq!(bitmap, expected_bitmap);
    }

    #[test]
    fn test_to_grayscale() {
        let gray_filename = test::get_expected_file_path(&ImageType::PNG, IMAGE_FROG_COLOR);
        let expected_filename = test::get_expected_file_path(&ImageType::PNG, IMAGE_FROG_GRAY);
        let mut result_bitmap = load_from_file(&gray_filename, &ImageType::PNG);
        result_bitmap.to_grayscale();
        let expected_bitmap = load_from_file(&expected_filename, &ImageType::PNG);
        assert_eq!(result_bitmap, expected_bitmap);
    }
}
