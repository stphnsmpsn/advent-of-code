use crate::submarine::{Movement, Submarine, SubmarineTrait};
use common::error::AocError;
use env_logger::Env;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

mod submarine;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    input_file: std::path::PathBuf,
}

pub fn main() -> Result<(), AocError> {
    let args = Opt::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Advent of Code: Day 02");
    info!(
        "The answer to part one is: {:?}",
        part_one(&args.input_file)?
    );
    info!(
        "The answer to part two is: {:?}",
        part_two(&args.input_file)?
    );
    Ok(())
}

fn part_one(path: &Path) -> Result<u32, AocError> {
    let file = File::open(path)?;
    let iter = BufReader::new(file).lines();
    let mut submarine = Submarine::default();
    for e in iter {
        submarine.travel_part_one(&Movement::from_str(e?.as_str())?)?;
    }
    let (x, y) = (submarine.get_x(), submarine.get_y());
    Ok(x * y)
}

fn part_two(path: &Path) -> Result<u32, AocError> {
    let file = File::open(path)?;
    let iter = BufReader::new(file).lines();
    let mut submarine = Submarine::default();
    for e in iter {
        submarine.travel_part_two(&Movement::from_str(e?.as_str())?)?;
    }
    let (x, y) = (submarine.get_x(), submarine.get_y());
    Ok(x * y)
}
