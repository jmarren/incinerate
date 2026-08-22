use crate::model::data::MnistItemBuilder;
use axum::extract::Multipart;
use burn::backend::Wgpu;
use maud::{Markup, html};
// const CANVAS_HEIGHT: usize = 392;
const CANVAS_WIDTH: usize = 392;

pub async fn accept_drawing(mut multipart: Multipart) -> Markup {
    let mut bytes = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("drawing") {
            bytes = field.bytes().await.unwrap().to_vec();
        }
    }

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
