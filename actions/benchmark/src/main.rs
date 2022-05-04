use crate::input::argument_parser::ArgumentParser;
use clap::Parser;
use log::{error};
use crate::input::validator::Validator;
use env_logger;
use crate::benchmark::benchmark::BenchmarkTest;
use crate::input::benchmark_config::BenchmarkConfig;

mod input;
mod benchmark;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = ArgumentParser::parse();

    let validator = Validator::from(args.path);
    let valid = validator.validate();
    if !valid {
        error!("{:?} is not a valid config file", validator.path);
        std::process::exit(0)
    }

    let config_reader = std::fs::File::open(validator.path)?;
    let mut benchmark_config: BenchmarkConfig = serde_yaml::from_reader(config_reader)?;
    let complete_path = benchmark_config.get_complete_path();

    benchmark_config.benchmarks.iter_mut().for_each(|benchmark| {
        benchmark.request_groups.iter_mut().for_each(|test_group| {
            test_group.base_path = Option::from(complete_path.clone());
        });
    });

    println!("Creating benchmarks");
    let benchmark_test = BenchmarkTest::new(benchmark_config.benchmarks.clone(), benchmark_config.get_goose_configuration());
    println!("Launching benchmarks");
    benchmark_test.launch().await?;

    Ok(())
}
