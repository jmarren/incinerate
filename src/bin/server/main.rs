use std::io::Cursor;

use axum::{
    Router,
    routing::{get, post},
};
use burn::{backend::Wgpu, data::dataset::vision::MnistItem};

mod markup;

// const CANVAS_HEIGHT: usize = 392;
const CANVAS_WIDTH: usize = 392;

async fn accept_drawing(body: axum::body::Bytes) {
    // println!("received drawing: {:?}", body);

    println!("body.len() = {}", body.len());

    let item = incinerate::data::MnistItemBuilder::from_bytes(&body.to_vec());

    type MyBackend = Wgpu<f32, i32>;
    let device = burn::backend::wgpu::WgpuDevice::default();
    incinerate::inference::infer::<MyBackend>("/tmp/guide", device, item);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(markup::index))
        .route("/drawings", post(accept_drawing));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
