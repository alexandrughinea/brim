use std::fs;

pub fn get_local_file_path_content(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}
