use std::io::Cursor;

// use crate::super::super::in

use axum::{
    Router,
    routing::{get, post},
};
use burn::data::dataset::vision::MnistItem;

mod markup;

// const CANVAS_HEIGHT: usize = 392;
const CANVAS_WIDTH: usize = 392;

async fn accept_drawing(body: axum::body::Bytes) {
    // println!("received drawing: {:?}", body);

    println!("body.len() = {}", body.len());

    let seeker = Cursor::new(body);

    let decoder = png::Decoder::new(seeker);
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size().unwrap()];

    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    // convert image to black and white only (not exactly grayscale (yet))
    let black_and_white: Vec<_> = bytes
        .chunks_exact(4)
        .into_iter()
        .map(|elt| if elt.iter().any(|x| *x != 0) { 255 } else { 0 })
        .collect();

    // convert to 2D array of size CANVAS_WIDTH X CANVAS_HEIGHT
    let bw_matrix: Vec<_> = black_and_white.chunks_exact(CANVAS_WIDTH).collect();

    let mut filled_count = 0;
    let mut empty_count = 0;

    black_and_white.iter().for_each(|elt| {
        if *elt == 255 {
            filled_count += 1;
        } else {
            empty_count += 1;
        }
    });

    println!("filled = {}, empty = {}", filled_count, empty_count);

    // the Mnist dataset contains 28x28 images
    //
    // to convert 392x392 to 28x28,
    // we will take the average of each 14x14 set of pixels from the input image

    let mut y = 0;

    let mut img = [[0.0; 28]; 28];
    // let mut img = vec![vec![0.0; 28]; 28];

    while y < bw_matrix.len() {
        let mut x = 0;

        while x < CANVAS_WIDTH {
            let mut found_drawing = false;
            let new_x = x.checked_div(14).unwrap_or(0);
            let new_y = y.checked_div(14).unwrap_or(0);

            'outer: for i in 0..14 {
                for j in 0..14 {
                    let x1 = x + i;
                    let y1 = y + j;

                    if bw_matrix[y1][x1] != 0 {
                        found_drawing = true;
                        break 'outer;
                    }
                }
            }

            if found_drawing {
                img[new_y][new_x] = 255.0;
            }

            x += 14;
        }

        y += 14;
    }

    for elt in img {
        println!("{:?}", elt);
    }

    let item = MnistItem {
        // Image as a 2D array of floats.
        image: img.into(),

        // Label of the image.
        label: 7,
    };

    // inference

    // crate::inference::infer::<MyBackend>(
    //     artifact_dir,
    //     device,
    //     burn::data::dataset::vision::MnistDataset::test()
    //         .get(44)
    //         .unwrap(),
    // );
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
