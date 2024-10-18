use reqwest::Client;
use serde_json::Value;
use std::path::Path;
use crate::uploader::FileUploader;
use crate::utils::extract_json_string;

pub struct GoFile {
    client: Client,
    file_uploader: FileUploader,
}

impl GoFile {
    pub fn new() -> Self {
        GoFile {
            client: Client::new(),
            file_uploader: FileUploader::new(),
        }
    }

    async fn get_best_server(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response: Value = self.client
            .get("https://api.gofile.io/servers")
            .send()
            .await?
            .json()
            .await?;

        if response["status"] != "ok" {
            return Err("GoFile API returned non-ok status".into());
        }

        response["data"]["servers"]
            .as_array()
            .and_then(|servers| servers.first())
            .and_then(|server| server["name"].as_str())
            .map(String::from)
            .ok_or_else(|| "No servers found in the response".into())
    }

    pub async fn upload_file(&self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let server = self.get_best_server().await?;
        let url = format!("https://{}.gofile.io/contents/uploadfile", server);

        let response = self.file_uploader.upload_file(&url, file_path).await?;
        let response_json: Value = response.json().await?;

        extract_json_string(&response_json, &["data", "downloadPage"])
    }
}
