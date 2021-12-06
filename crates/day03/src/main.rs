use crate::repo::in_memory_repo::InMemoryRepo;
use crate::ship_computer::{diagnostic::DiagnosticEntry, ShipComputer};
use common::error::AocError;
use env_logger::Env;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

mod repo;
mod ship_computer;
mod util;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    input_file: std::path::PathBuf,
}

pub fn main() -> Result<(), AocError> {
    let args = Opt::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Advent of Code: Day 03");
    info!("Part One: {:?}", part_one(&args.input_file)?);
    info!("Part Two: {:?}", part_two(&args.input_file)?);
    Ok(())
}

fn part_one(path: &Path) -> Result<u32, AocError> {
    let mut ship_computer = ShipComputer::new(Box::new(InMemoryRepo::default()));

    for e in BufReader::new(File::open(path)?).lines() {
        ship_computer.add_diagnostic_entry(DiagnosticEntry::from_str(e?.as_str())?)?;
    }

    ship_computer.calculate_power_consumption()
}

fn part_two(path: &Path) -> Result<u32, AocError> {
    let mut ship_computer = ShipComputer::new(Box::new(InMemoryRepo::default()));

    for e in BufReader::new(File::open(path)?).lines() {
        ship_computer.add_diagnostic_entry(DiagnosticEntry::from_str(e?.as_str())?)?;
    }

    ship_computer.calculate_life_support()
}
