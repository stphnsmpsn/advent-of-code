use crate::depth::rate_of_depth_increase;
use common::error::AocError;
use env_logger::Env;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use structopt::StructOpt;

mod depth;

const SLIDING_WINDOW_SIZE: usize = 3;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    input_file: std::path::PathBuf,
}

pub fn main() -> Result<(), AocError> {
    let args = Opt::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Advent of Code: Day 01");
    info!("The answer to part one is: {}", part_one(&args.input_file)?);
    info!("The answer to part two is: {}", part_two(&args.input_file)?);
    Ok(())
}

fn part_one(path: &Path) -> Result<u32, AocError> {
    let file = File::open(path)?;
    let iter = BufReader::new(file).lines();
    rate_of_depth_increase(iter)
}

//todo: avoid reading entire input file into memory (unsafe) so as not to potentially exhaust
// resources on large datasets
fn part_two(path: &Path) -> Result<u32, AocError> {
    let file = File::open(path)?;

    let v = BufReader::new(file)
        .lines()
        .map(|n| Ok(n?.parse::<u32>()?))
        .collect::<Result<Vec<u32>, AocError>>()?;

    let iter = v.windows(SLIDING_WINDOW_SIZE);
    rate_of_depth_increase(iter)
}
