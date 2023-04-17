use std::io::Cursor;
use url::form_urlencoded::{byte_serialize, parse};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let key: String = byte_serialize(byte_serialize("HEC/HECAgentKotlin/release/22.12.12.0.00/2/HecAgent-v22.12.12.0.2-debug-signed.apk".as_bytes()).collect::<String>().as_bytes()).collect();
    let url = format!(
        "https://artifactory.tc.need-replace.com/ui/api/v1/download?repoKey=need-replace-release-generic&path={}&isNativeBrowsing=true",
        key
    );
    println!("url:{}", url);
    fetch_url(url.to_string(), "siriel.apk".to_string())
        .await
        .unwrap();
    //https://artifactory.tc.need-replace.com/ui/api/v1/download?repoKey=need-replace-release-generic&path=HEC%252FHECAgentKotlin%252Frelease%252F22.12.12.0.00%252F2%252Fsample-app-v22.12.12.0.2-debug-signed.apk&isNativeBrowsing=true
    //https://artifactory.tc.need-replace.com/ui/api/v1/download?repoKey=need-replace-release-generic&path=HEC%2FHECAgentKotlin%2Frelease%2F22.12.12.0.00%2F2%2FHecAgent-v22.12.12.0.2-debug-signed.apk&isNativeBrowsing=true
}
