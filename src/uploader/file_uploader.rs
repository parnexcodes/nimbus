use std::path::Path;
use tokio::fs::File;
use reqwest::multipart::{Form, Part};
use crate::utils::get_file_name;
use indicatif::{ProgressBar, ProgressStyle};
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct FileUploader {
    client: reqwest::Client,
}

struct ProgressStream<S> {
    inner: S,
    progress_bar: ProgressBar,
    last_update: Instant,
    bytes_since_last_update: u64,
}

impl<S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Unpin> Stream for ProgressStream<S> {
    type Item = Result<bytes::Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                let chunk_len = chunk.len() as u64;
                self.progress_bar.inc(chunk_len);
                self.bytes_since_last_update += chunk_len;

                let now = Instant::now();
                if now.duration_since(self.last_update) >= Duration::from_secs(1) {
                    let elapsed = now.duration_since(self.last_update).as_secs_f64();
                    let speed = self.bytes_since_last_update as f64 / elapsed;
                    let speed_str = if speed >= 1_000_000.0 {
                        format!("{:.2} MB/s", speed / 1_000_000.0)
                    } else {
                        format!("{:.2} KB/s", speed / 1_000.0)
                    };
                    self.progress_bar.set_message(speed_str);
                    self.last_update = now;
                    self.bytes_since_last_update = 0;
                }

                Poll::Ready(Some(Ok(chunk)))
            }
            other => other,
        }
    }
}

impl FileUploader {
    pub fn new() -> Self {
        FileUploader {
            client: reqwest::Client::new(),
        }
    }

    pub async fn upload_file(&self, url: &str, file_path: &Path) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let file = File::open(file_path).await?;
        let file_size = file.metadata().await?.len();
        let file_name = get_file_name(file_path)?;

        let pb = ProgressBar::new(file_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"));

        let stream = tokio_util::io::ReaderStream::new(file);
        let progress_stream = ProgressStream {
            inner: stream,
            progress_bar: pb.clone(),
            last_update: Instant::now(),
            bytes_since_last_update: 0,
        };

        let part = Part::stream(reqwest::Body::wrap_stream(progress_stream))
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let form = Form::new().part("file", part);

        let response = self.client
            .post(url)
            .multipart(form)
            .send()
            .await?;

        pb.finish_with_message("Upload complete");

        Ok(response)
    }
}
