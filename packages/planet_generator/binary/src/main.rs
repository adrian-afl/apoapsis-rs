mod cli_args;
mod craters;
mod erosion;
mod generate_terrain;
mod math_util;
mod save_binary_maps;

use crate::cli_args::CLIArgs;
use crate::generate_terrain::{generate_terrain, generate_water};
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use universe_simulation::body_definitions::load_body_data;

fn main() {
    let cli_args = CLIArgs::parse();
    let input = load_body_data(&cli_args.input);

    let start = Instant::now();

    let mut dir = PathBuf::from(cli_args.input);
    dir.pop();
    dir.push(&input.generator_config.as_ref().unwrap().out_dir);

    generate_terrain(dir.to_str().unwrap(), &input);
    generate_water(dir.to_str().unwrap(), &input);

    let duration = start.elapsed();
    println!("Generation finished in: {:?}", duration);
}
