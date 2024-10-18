use std::path::Path;

pub fn get_file_name(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    path.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Failed to extract file name".into())
        .map(|s| s.to_string())
}
