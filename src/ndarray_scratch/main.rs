use ndarray::{OwnedRepr, ViewRepr, prelude::*};

fn main() {
    println!("ndarray scratch");

    // try_slice();
    // try_create_arrays();
    // try_add_arrays();
    // try_azip();
    // try_addition();
    try_multiply();
}

fn try_multiply() {
    let data1 = vec![1.0, 3.0, 6.0, 7.0, 15.0, 16.0, 3.0, 40.0];
    let arr1 = Array2::from_shape_vec((2, 4), data1).unwrap();
    let data2 = vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0];
    let arr2 = Array2::from_shape_vec((2, 4), data2).unwrap();
    println!("arr1 = {}", arr1);
    println!("arr2 = {}", arr2);

    println!("arr1 * arr2 = {}", arr1 * arr2);
    // println!("arr1.dot(arr2) = {}", arr1.dot(&arr2));
}

fn try_slice() {
    let data1 = vec![1.0, 3.0, 6.0, 7.0, 15.0, 16.0, 3.0, 40.0];

    let arr = Array4::<f32>::from_shape_vec((2, 2, 2, 1), data1).unwrap();

    println!("arr = {}", arr);
    let s1 = arr.slice(s![1, 0..2, 0, 0]);

    println!("s1 = {}", s1);
}

fn try_create_arrays() {
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

fn try_addition() {
    let data1 = vec![1.0, 3.0, 6.0];
    let mut arr1 = Array2::<f32>::from_shape_vec((3, 1), data1).unwrap();
    let data2 = vec![6.0, 15.0, 2.0];
    let arr2 = Array2::<f32>::from_shape_vec((3, 1), data2).unwrap();

    arr1 = arr1 + arr2;

    dbg!(arr1);
}

fn try_azip() {
    let data1 = vec![1.0, 3.0, 6.0];
    let mut arr1 = Array2::<f32>::from_shape_vec((3, 1), data1).unwrap();
    let data2 = vec![6.0, 15.0, 2.0];
    let arr2 = Array2::<f32>::from_shape_vec((3, 1), data2).unwrap();

    azip!((a in &mut arr1, b in &arr2) *a += *b);

    dbg!(arr1);
}

fn try_add_arrays() {
    let data1 = vec![1.0, 3.0, 6.0];
    let mut arr1 = Array2::<f32>::from_shape_vec((3, 1), data1).unwrap();
    let data2 = vec![6.0, 15.0, 2.0];
    let arr2 = Array2::<f32>::from_shape_vec((3, 1), data2).unwrap();

    add_arrays(arr1.view_mut(), arr2.view());

    dbg!(arr1);
}

fn add_arrays(mut arr1: ArrayViewMut2<f32>, arr2: ArrayView2<f32>) {
    for row in 0..arr1.nrows() {
        for col in 0..arr1.ncols() {
            // NOTE: this can be made faster by using the following unsafe code to skip bounds
            // checking
            // unsafe {
            //     *arr1.uget_mut([row, col]) += arr2.uget([row, col]);
            // }
            arr1[[row, col]] += arr2[[row, col]];
        }
    }
}
