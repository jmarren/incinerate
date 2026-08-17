use clap::Parser;
use incinerate::model::cli::Cli;

fn main() {
    incinerate::model::cli::run(Cli::parse());
}
