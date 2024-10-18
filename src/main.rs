use crate::uploader::gofile_uploader::upload_to_gofile;
use crate::utils::init_logger;
use clap::Parser;
use colored::Colorize;
use log::info;
use std::path::Path;

mod sites;
mod uploader;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file or folder to upload
    #[arg(short, long)]
    path: String,

    /// List of file sharing sites to upload to (comma-separated)
    #[arg(short, long)]
    sites: Vec<String>,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    // Initialize logger
    init_logger("info");

    // Parse command-line arguments
    let args = Args::parse();

    info!("Starting nimbus");

    println!("{}", r#"
 _   _ _           _               
| \ | (_)         | |              
|  \| |_ _ __ ___ | |__  _   _ ___ 
| . ` | | '_ ` _ \| '_ \| | | / __|
| |\  | | | | | | | |_) | |_| \__ \
\_| \_/_|_| |_| |_|_.__/ \__,_|___/
    "#.cyan());
    println!("{}", "File/Folder Upload Tool".green().bold());
    println!("Path: {}", args.path);
    println!("Sites: {}", args.sites.join(", "));
    println!("Verbose: {}", args.verbose);

    for site in &args.sites {
        match site.to_lowercase().as_str() {
            "gofile" => {
                println!("\nUploading to GoFile...");
                match upload_to_gofile(Path::new(&args.path)).await {
                    Ok(_) => info!("Successfully uploaded to GoFile"),
                    Err(e) => {
                        eprintln!("Error uploading to GoFile: {}", e);
                        info!("Failed to upload to GoFile: {}", e);
                    }
                }
            }
            // Add other sites here
            _ => {
                eprintln!("Unsupported site: {}", site);
                info!("Attempted to upload to unsupported site: {}", site);
            }
        }
    }

    info!("Upload process completed");
}
