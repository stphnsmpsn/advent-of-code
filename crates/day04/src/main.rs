use crate::bingo::board::{Bingo, Board};
use crate::bingo::board_reader::BoardReader;
use chrono::Utc;
use common::error::AocError;
use env_logger::Env;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use structopt::StructOpt;

mod bingo;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    input_file: std::path::PathBuf,
}

pub fn main() -> Result<(), AocError> {
    let args = Opt::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Advent of Code: Day 04");

    let start = Utc::now();
    info!(
        "The answer to part one is: {:?} (took: {}).",
        part_one(&args.input_file)?.ok_or_else(|| AocError::EmptyOption)?,
        Utc::now().signed_duration_since(start)
    );

    info!(
        "The answer to part two is: {:?} (took: {}).",
        part_two(&args.input_file)?.ok_or_else(|| AocError::EmptyOption)?,
        Utc::now().signed_duration_since(start)
    );

    Ok(())
}

fn part_one(path: &Path) -> Result<Option<u32>, AocError> {
    let file = File::open(path)?;
    let mut br = BufReader::new(file);

    let mut move_string = String::new();
    br.read_line(&mut move_string)?;

    let move_list = move_string
        .split(',')
        .map(|e| Ok(e.trim().parse::<u32>()?))
        .collect::<Result<Vec<u32>, AocError>>()?;

    let board_reader = BoardReader::new(br);
    let mut boards = board_reader.into_iter().collect::<Vec<Board>>();

    for val in move_list {
        for board in boards.iter_mut() {
            match board.add(val) {
                None => {}
                Some(bingo) => {
                    return Ok(Some(bingo.score()));
                }
            }
        }
    }

    Ok(None)
}

fn part_two(path: &Path) -> Result<Option<u32>, AocError> {
    let file = File::open(path)?;
    let mut br = BufReader::new(file);

    let mut move_string = String::new();
    br.read_line(&mut move_string)?;

    let move_list = move_string
        .split(',')
        .map(|e| Ok(e.trim().parse::<u32>()?))
        .collect::<Result<Vec<u32>, AocError>>()?;

    let board_reader = BoardReader::new(br);
    let mut boards = board_reader.into_iter().collect::<Vec<Board>>();

    let mut last_winning_board: Option<(usize, Bingo)> = None;
    for val in move_list {
        for (idx, board) in boards.iter_mut().enumerate() {
            match board.add(val) {
                None => {}
                Some(bingo) => {
                    last_winning_board = Some((idx, bingo));
                }
            }
        }
    }

    match last_winning_board {
        None => Ok(None),
        Some((_, bingo)) => Ok(Some(bingo.score())),
    }
}
