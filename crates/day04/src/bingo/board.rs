use common::error::AocError;
use std::convert::TryFrom;

const BOARD_SIZE: usize = 25;

pub struct Bingo(u32);

impl Bingo {
    pub fn score(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct BoardSpace {
    val: u32,
    marked: bool,
}

impl TryFrom<&str> for BoardSpace {
    type Error = AocError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            val: value.trim().parse::<u32>()?,
            marked: false,
        })
    }
}

#[derive(Debug)]
pub struct Board {
    vals: Vec<BoardSpace>,
    row_length: usize,
    pub won: bool,
}

impl Board {
    pub fn new(vals: Vec<BoardSpace>, row_length: usize) -> Result<Self, AocError> {
        if vals.len() != BOARD_SIZE {
            Err(AocError::InputError("Improper Board Format".to_string()))
        } else {
            Ok(Self {
                vals,
                row_length,
                won: false,
            })
        }
    }

    pub fn add(&mut self, val: u32) -> Option<Bingo> {
        if !self.won {
            for v in self.vals.iter_mut() {
                if v.val == val {
                    v.marked = true;
                }
            }

            if self.evaluate() {
                return Some(Bingo(self.calculate_score(val)));
            }
        }
        None
    }

    fn evaluate(&mut self) -> bool {
        let mut won = true;

        for i in 0..(self.vals.len() / self.row_length) {
            // check ith row
            won = true;
            for j in 0..self.row_length {
                if !self.vals[j].marked {
                    won = false;
                    break;
                }
            }

            if won {
                self.won = true;
                return true;
            }

            // check ith column
            won = true;
            let mut j = i;
            for _ in 0..self.row_length {
                if !self.vals[j].marked {
                    won = false;
                    break;
                }
                j += self.row_length
            }
            if won {
                self.won = true;
                return true;
            }
        }

        won
    }

    fn calculate_score(&self, last_called: u32) -> u32 {
        last_called
            * self
                .vals
                .iter()
                .fold(0, |acc, x| if x.marked { acc } else { acc + x.val })
    }
}
