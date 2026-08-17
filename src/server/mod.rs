mod markup;

use crate::model::data::MnistItemBuilder;
use axum::{
    Router,
    extract::Multipart,
    routing::{get, post},
};
use burn::backend::Wgpu;
use maud::{Markup, html};

// const CANVAS_HEIGHT: usize = 392;
const CANVAS_WIDTH: usize = 392;

async fn accept_drawing(mut multipart: Multipart) -> Markup {
    let mut bytes = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("drawing") {
            bytes = field.bytes().await.unwrap().to_vec();
        }
    }

    println!("body.len() = {}", bytes.len());

    let item = MnistItemBuilder::from_bytes(&bytes);

    if let Err(e) = MnistItemBuilder::save_png(&item, "drawings/drawing.png") {
        eprintln!("failed to save drawing png: {e}");
    }

    type MyBackend = Wgpu<f32, i32>;
    let device = burn::backend::wgpu::WgpuDevice::default();
    let prediction = crate::model::inference::infer::<MyBackend>("/tmp/guide", device, item);

    html! {
        (prediction)
    }
}

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(markup::index))
        .route("/drawings", post(accept_drawing));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
