// use super::bitmap::ImageType;

// const EXPECTED_IMAGE_FOLDER_PATH: &str = "tests/test_images";

// pub fn get_expected_file_path(image_type: &ImageType) -> std::string::String {
//     let folder_path = std::path::Path::new(EXPECTED_IMAGE_FOLDER_PATH);
//     let file_path = folder_path
//         .join(TEST_IMAGE_NAME)
//         .with_extension(match image_type {
//             ImageType::PNG => "png",
//         });
//     return file_path.to_string_lossy().to_string();
// }