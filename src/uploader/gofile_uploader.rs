use crate::sites::gofile::GoFile;
use std::path::Path;
use log::{info, error};

pub async fn upload_to_gofile(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let gofile = GoFile::new();
    info!("Uploading file to GoFile...");
    match gofile.upload_file(path).await {
        Ok(download_url) => {
            println!("\nFile uploaded to GoFile. Download URL: {}", download_url);
            Ok(())
        },
        Err(e) => {
            error!("Failed to upload to GoFile: {}", e);
            Err(e)
        }
    }
}
