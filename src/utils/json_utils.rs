use serde_json::Value;

pub fn extract_json_string(
    json: &Value,
    path: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut current = json;
    for &key in path {
        current = &current[key];
    }
    current
        .as_str()
        .ok_or_else(|| format!("Failed to extract string from JSON path: {:?}", path).into())
        .map(|s| s.to_string())
}
