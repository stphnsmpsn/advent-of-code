use crate::bingo::board::{Board, BoardSpace};
use common::error::AocError;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SPACES_PER_ROW: usize = 5;

pub struct BoardReader {
    br: BufReader<File>,
}

impl BoardReader {
    pub fn new(br: BufReader<File>) -> Self {
        Self { br }
    }
}

impl Iterator for BoardReader {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::default();

        // consume the empty line that separates the boards (or get EOF)
        if let Ok(e) = self.br.read_line(&mut line) {
            // check if we've reached EOF
            if e == 0 {
                return None;
            }

            // create storage for board
            let mut res: Vec<BoardSpace> = vec![];

            for _ in 0..SPACES_PER_ROW {
                line.clear();
                self.br.read_line(&mut line).ok()?;
                res.append(
                    line.split_whitespace()
                        .map(BoardSpace::try_from)
                        .collect::<Result<Vec<BoardSpace>, AocError>>()
                        .ok()?
                        .as_mut(), // todo: handle and return none
                );
            }

            Board::new(res, SPACES_PER_ROW).ok()
        } else {
            None
        }
    }
}
