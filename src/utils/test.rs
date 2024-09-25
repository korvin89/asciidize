use super::bitmap;

const EXPECTED_IMAGE_FOLDER_PATH: &str = "tests/test_images";
const TEST_IMAGE_FOLDER_PATH: &str = "tests//tmp";

#[must_use]
pub fn get_expected_file_path(
    image_type: &bitmap::ImageType,
    image_name: &str,
) -> std::string::String {
    let folder_path = std::path::Path::new(EXPECTED_IMAGE_FOLDER_PATH);
    let file_path = folder_path
        .join(image_name)
        .with_extension(match image_type {
            bitmap::ImageType::PNG => "png",
        });
    return file_path.to_string_lossy().to_string();
}

/// # Panics
///
/// Will panic if could not create folder by `folder_path`
#[must_use]
pub fn get_test_file_path(image_type: &bitmap::ImageType, image_name: &str) -> std::string::String {
    let folder_path = std::path::Path::new(TEST_IMAGE_FOLDER_PATH);
    std::fs::create_dir_all(folder_path).unwrap();

    let file_path = folder_path
        .join(image_name)
        .with_extension(match image_type {
            bitmap::ImageType::PNG => "png",
        });
    return file_path.to_string_lossy().to_string();
}
