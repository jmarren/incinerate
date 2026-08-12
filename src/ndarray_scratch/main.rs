use ndarray::{OwnedRepr, ViewRepr, prelude::*};

fn main() {
    println!("ndarray scratch");

    // Owned 2 x 3 matrix of zeros -- Owned Representation
    let arr = ArrayBase::<OwnedRepr<f32>, Ix2>::zeros((2, 3));

    dbg!(arr);

    // Not-owned 2 x 3 matrix of zeros -- View Representation
    let data = vec![0.0; 6];
    let arr = ArrayBase::<ViewRepr<&f32>, Ix2>::from_shape((2, 3), &data).unwrap();

    dbg!(arr);

    // Mutable Not-owned 2 x 3 matrix of zeros -- View Representation
    let mut data = vec![0.0; 6];
    let arr = ArrayBase::<ViewRepr<&mut f32>, Ix2>::from_shape((2, 3), &mut data).unwrap();

    dbg!(arr);

    // Using Array2 syntax for owned representation
    let arr1 = Array2::<f32>::zeros((2, 3));
    println!("{}", arr1);

    // Using ArrayView2 syntax for view representation
    let arr = ArrayView2::<f32>::from_shape((2, 3), &data).unwrap();
    dbg!(arr);

    // Using ArrayViewMut2 syntax for mutable view representation
    let arr = ArrayViewMut2::<f32>::from_shape((2, 3), &mut data).unwrap();
    dbg!(arr);

    // create a view from an owned rep
    let arr1_view = arr1.view();
    dbg!(arr1_view);

    // dynamic dimensions
    let arr = ArrayD::<f32>::zeros(IxDyn(&[2, 3]));
    dbg!(arr);
}

fn add_arrays() {}
