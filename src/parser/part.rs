use reqwest;
use std::io::Cursor;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn download_zip(url: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let content = Cursor::new(response.bytes().await?);
    log::info!("{:?}", content);
    Ok(())
}

pub async fn test(id: &str) {
    download_zip(format!("{}{}", "http://localhost:3000/bundle/", id))
        .await
        .unwrap();

    log::info!("parsing part {}", id);
}
