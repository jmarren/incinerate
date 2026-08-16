use clap::{Parser, ValueEnum};

use burn::{
    backend::{Autodiff, Wgpu},
    data::dataset::Dataset,
    optim::AdamConfig,
};
use incinerate::{model::ModelConfig, training::TrainingConfig};

#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    #[arg(value_enum)]
    action: Action,
}
#[derive(ValueEnum, Clone, Debug)]
enum Action {
    // #[value(POSSIBLE VALUE ATTRIBUTE)]
    Infer,
    // #[value(POSSIBLE VALUE ATTRIBUTE)]
    Train,
}

fn main() {
    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "/tmp/guide";

    let args = Cli::parse();
    println!("args = {:?}", args);

    // let forty_four = burn::data::dataset::vision::MnistDataset::test()
    //     .get(44)
    //     .unwrap();

    // println!("

    match args.action {
        Action::Train => {
            incinerate::training::train::<MyAutodiffBackend>(
                artifact_dir,
                TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
                device.clone(),
            );
        }
        Action::Infer => {
            incinerate::inference::infer::<MyBackend>(
                artifact_dir,
                device,
                burn::data::dataset::vision::MnistDataset::test()
                    .get(44)
                    .unwrap(),
            );
        }
    }
}
