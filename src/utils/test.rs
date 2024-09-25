const EXPECTED_IMAGE_FOLDER_PATH: &str = "tests/test_images";

#[must_use]
pub fn get_expected_file_path(image_name: &str) -> std::string::String {
    let folder_path = std::path::Path::new(EXPECTED_IMAGE_FOLDER_PATH);
    let file_path = folder_path.join(image_name);
    return file_path.to_string_lossy().to_string();
}

#[must_use]
pub fn get_test_file_path(image_name: &str) -> std::string::String {
    let folder_path = assert_fs::fixture::TempDir::new()
        .unwrap()
        .into_persistent();

    let file_path = folder_path.join(image_name);
    return file_path.to_string_lossy().to_string();
}
