# lib_ngrok

rust crate for downloading and running an ngrok binary.

```rs
use lib_ngrok::download;
 
async fn download() -> Result<(), anyhow::Error> {
    let ngrok_path = &Path::new("ngrok");
    download::to_path(ngrok_path).await?;
}
```