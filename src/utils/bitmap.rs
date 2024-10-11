use image::GenericImageView;

#[derive(Debug, PartialEq, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    #[must_use]
    pub fn get_intensity(&self) -> u8 {
        return self.r / 3 + self.g / 3 + self.b / 3 + (self.r % 3 + self.g % 3 + self.b % 3) / 3;
    }

    pub fn to_grayscale(&mut self) {
        let value = self.get_intensity();
        self.r = value;
        self.g = value;
        self.b = value;
    }

    pub fn to_black_and_white(&mut self, threshold: u8) {
        let value = self.get_intensity();
        if value > threshold {
            self.r = 255;
            self.g = 255;
            self.b = 255;
        } else {
            self.r = 0;
            self.g = 0;
            self.b = 0;
        }
    }

    pub fn to_invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
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

    pub fn to_black_and_white(&mut self) {
        let mut max_intensity = 0;
        let mut min_intensity = 255;

        for y in 0..self.height {
            for x in 0..self.width {
                let intensity = self.pixels[y as usize][x as usize].get_intensity();
                if intensity > max_intensity {
                    max_intensity = intensity;
                }
                if intensity < min_intensity {
                    min_intensity = intensity;
                }
            }
        }

        let threshold = (max_intensity + min_intensity) / 2;

        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[y as usize][x as usize].to_black_and_white(threshold);
            }
        }
    }

    pub fn to_invert(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[y as usize][x as usize].to_invert();
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

    const IMAGE_2X2: &str = "2x2.png";
    const IMAGE_FROG_COLOR: &str = "frog_32x32_color.png";
    const IMAGE_FROG_GRAY: &str = "frog_32x32_gray.png";
    const IMAGE_FROG_BW: &str = "frog_32x32_bw.png";
    const IMAGE_FROG_INVERT: &str = "frog_32x32_invert.png";

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
        let filename = test::get_test_file_path(IMAGE_2X2);
        save_to_file(&bitmap, &filename, &ImageType::PNG);

        let expected_filename = test::get_expected_file_path(IMAGE_2X2);
        assert_file_content(&filename, &expected_filename);

        std::fs::remove_file(&filename).unwrap();
    }

    #[test]
    fn test_load_from_file() {
        let expected_filename = test::get_expected_file_path(IMAGE_2X2);
        let bitmap = load_from_file(&expected_filename, &ImageType::PNG);
        let expected_bitmap = get_bitmap();
        assert_eq!(bitmap, expected_bitmap);
    }

    #[test]
    fn test_to_grayscale() {
        let color_filename = test::get_expected_file_path(IMAGE_FROG_COLOR);
        let expected_filename = test::get_expected_file_path(IMAGE_FROG_GRAY);
        let expected_bitmap = load_from_file(&expected_filename, &ImageType::PNG);

        let mut bitmap = load_from_file(&color_filename, &ImageType::PNG);
        bitmap.to_grayscale();

        assert_eq!(bitmap, expected_bitmap);
    }

    #[test]
    fn test_to_black_and_white() {
        let color_filename = test::get_expected_file_path(IMAGE_FROG_COLOR);
        let expected_filename = test::get_expected_file_path(IMAGE_FROG_BW);
        let expected_bitmap = load_from_file(&expected_filename, &ImageType::PNG);

        let mut bitmap = load_from_file(&color_filename, &ImageType::PNG);
        bitmap.to_black_and_white();

        assert_eq!(bitmap, expected_bitmap);
    }

    #[test]
    fn test_to_invert() {
        let color_filename = test::get_expected_file_path(IMAGE_FROG_COLOR);
        let expected_filename = test::get_expected_file_path(IMAGE_FROG_INVERT);
        let expected_bitmap = load_from_file(&expected_filename, &ImageType::PNG);

        let mut bitmap = load_from_file(&color_filename, &ImageType::PNG);
        bitmap.to_invert();

        assert_eq!(bitmap, expected_bitmap);
    }
}
