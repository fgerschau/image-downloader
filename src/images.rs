use crate::files::Occurrence;
use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use indicatif::ProgressBar;
use tokio::fs::File;
use tokio::io::copy;
use tokio_util;

fn get_client() -> Client<HttpsConnector<hyper::client::HttpConnector>> {
    let https = HttpsConnector::new();
    Client::builder().build(https)
}

fn to_tokio_async_read(r: impl futures::io::AsyncRead) -> impl tokio::io::AsyncRead {
    tokio_util::compat::FuturesAsyncReadCompatExt::compat(r)
}

#[tokio::main]
pub async fn download_images(occurrences: &Vec<Occurrence>, folder: String) -> Result<u32> {
    let client = get_client();
    let mut success_count = 0;

    let bar = ProgressBar::new(occurrences.len() as u64);
    for occurrence in occurrences {
        bar.inc(1);
        let uri = occurrence.text.parse::<hyper::Uri>()?;
        let request = Request::get(uri)
            .header("User-Agent", "Mozilla/5.0") // Some websites require a user agent
            .body(Body::empty())?;

        let response = client.request(request).await?;

        if response.status().is_success() {
            success_count += 1;
        }

        let file_name = match occurrence.text.split('/').last() {
            Some(file_name) => file_name,
            None => {
                println!("Error: no file name found in {}", occurrence.text);
                continue;
            }
        };

        let file_path = format!("{}/{}", folder, file_name);

        let mut file = File::create(&file_path).await?;

        let stream = response
            .into_body()
            .map(|result| {
                result.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error!"))
            })
            .into_async_read();

        let mut tokio_async_read = to_tokio_async_read(stream);

        copy(&mut tokio_async_read, &mut file).await?;
    }

    Ok(success_count)
}
