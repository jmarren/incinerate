use burn::{
    backend::{Autodiff, Wgpu},
    data::dataset::Dataset,
    optim::AdamConfig,
};
use clap::{Parser, ValueEnum};

use crate::model::{network::NetworkConfig, training::TrainingConfig};

#[derive(Parser, Debug)]
pub struct Cli {
    /// The pattern to look for
    #[arg(value_enum)]
    action: Action,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Action {
    // #[value(POSSIBLE VALUE ATTRIBUTE)]
    Infer,
    // #[value(POSSIBLE VALUE ATTRIBUTE)]
    Train,
}

pub fn run(args: Cli) {
    // use Wgpu as backend
    type MyBackend = Wgpu<f32, i32>;
    // create an Autodiff type that decorates the backend to allow for autodiff
    // (automatic differentiation)
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "/tmp/guide";

    println!("args = {:?}", args);

    match args.action {
        Action::Train => {
            crate::model::training::train::<MyAutodiffBackend>(
                artifact_dir,
                TrainingConfig::new(NetworkConfig::new(10, 512), AdamConfig::new()),
                device.clone(),
            );
        }
        Action::Infer => {
            let test_value = burn::data::dataset::vision::MnistDataset::test()
                .get(56)
                .unwrap();

            let result = crate::model::inference::infer::<MyBackend>(
                artifact_dir,
                device,
                test_value.clone(),
            );

            println!("expected = {:?}, result = {:?}", test_value.label, result);
        }
    }
}
