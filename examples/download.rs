use lib_ngrok::download;

#[tokio::main]
async fn main() {

    let ngrok_path = &Path::new("ngrok");

    let download = download::bin(&Path::new("ngrok")).await;
}