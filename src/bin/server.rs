#[tokio::main]
async fn main() {
    incinerate::server::start_server().await;
}
