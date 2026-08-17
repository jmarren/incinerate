use std::io::Cursor;

use burn::{
    data::{dataloader::batcher::Batcher, dataset::vision::MnistItem},
    prelude::*,
};

#[derive(Clone, Default)]
pub struct MnistBatcher {}

#[derive(Clone, Debug)]
pub struct MnistBatch<B: Backend> {
    pub images: Tensor<B, 3>,
    pub targets: Tensor<B, 1, Int>,
}

impl<B: Backend> Batcher<B, MnistItem, MnistBatch<B>> for MnistBatcher {
    fn batch(&self, items: Vec<MnistItem>, device: &B::Device) -> MnistBatch<B> {
        // for each item
        let images = items
            .iter()
            // map the image to a tensor and convert the elements to floats
            .map(|item| TensorData::from(item.image).convert::<B::FloatElem>())
            // create a tensor from the data for the device
            .map(|data| Tensor::<B, 2>::from_data(data, device))
            // reshape the tensor to 1x28x28
            .map(|tensor| tensor.reshape([1, 28, 28]))
            // Normalize: scale between [0,1] and make the mean=0 and std=1
            // values mean=0.1307,std=0.3081 are from the PyTorch MNIST example
            // https://github.com/pytorch/examples/blob/54f4572509891883a947411fd7239237dd2a39c3/mnist/main.py#L122
            .map(|tensor| ((tensor / 255) - 0.1307) / 0.3081)
            .collect();

        // for each item
        let targets = items
            .iter()
            // map the item to a 1D tensor
            .map(|item| {
                Tensor::<B, 1, Int>::from_data([(item.label as i64).elem::<B::IntElem>()], device)
            })
            .collect();

        // concatenate the tensors into one tensor along the 0 dimension
        let images = Tensor::cat(images, 0);
        let targets = Tensor::cat(targets, 0);

        MnistBatch { images, targets }
    }
}

const CANVAS_WIDTH: usize = 392;

// fn png_decoder_from_bytes(bytes: &[u8]) -> png::Decoder<Cursor<&[u8]>> {
//     let seeker = Cursor::new(bytes);
//     png::Decoder::new(seeker)
// }

pub struct MnistItemBuilder {}

fn rgba_to_bw(bytes: &[u8]) -> Vec<u8> {
    bytes
        .chunks_exact(4)
        .into_iter()
        .map(|elt| if elt.iter().any(|x| *x != 0) { 255 } else { 0 })
        .collect()
}

fn resize_to_mnist(bytes_matrix: Vec<&[u8]>) -> [[f32; 28]; 28] {
    let mut y = 0;

    let mut img = [[0.0; 28]; 28];
    // let mut img = vec![vec![0.0; 28]; 28];

    while y < bytes_matrix.len() {
        let mut x = 0;

        while x < CANVAS_WIDTH {
            let mut found_drawing = false;
            let new_x = x.checked_div(14).unwrap_or(0);
            let new_y = y.checked_div(14).unwrap_or(0);

            'outer: for i in 0..14 {
                for j in 0..14 {
                    let x1 = x + i;
                    let y1 = y + j;

                    if bytes_matrix[y1][x1] != 0 {
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

    img.iter_mut().for_each(|elt| {
        elt.iter_mut().for_each(|e| {
            if *e == 255.0 {
                *e = 0.0;
            } else {
                *e = 255.0;
            }
        })
    });

    img
}

impl MnistItemBuilder {
    pub fn from_bytes(bytes: &[u8]) -> MnistItem {
        let seeker = Cursor::new(bytes);

        let decoder = png::Decoder::new(seeker);
        let mut reader = decoder.read_info().unwrap();

        let mut buf = vec![0; reader.output_buffer_size().unwrap()];

        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];

        // convert from rgba to black and white (all 0 or 255)
        let black_and_white = rgba_to_bw(bytes);

        // convert to 2D array of size CANVAS_WIDTH X CANVAS_HEIGHT
        let bw_matrix: Vec<_> = black_and_white.chunks_exact(CANVAS_WIDTH).collect();

        // the Mnist dataset contains 28x28 images
        //
        // to convert 392x392 to 28x28,
        // we will take each 14x14 set of pixels from the input image
        // and use 255 if it contains any value other than 0.0
        let img = resize_to_mnist(bw_matrix);

        for elt in img {
            println!("{:?}", elt);
        }

        MnistItem {
            // Image as a 2D array of floats.
            image: img.into(),

            // Label of the image.
            label: 7,
        }
    }
}
