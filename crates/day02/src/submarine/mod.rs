use common::error::AocError;
use common::math::MyCheckedOps;
use std::str::FromStr;

#[derive(Default)]
pub struct Submarine {
    pos_x: u32,
    pos_y: u32,
    aim: u32,
}

pub trait SubmarineTrait {
    fn travel_part_one(&mut self, direction: &Movement) -> Result<(), AocError>;
    fn travel_part_two(&mut self, direction: &Movement) -> Result<(), AocError>;
}

impl SubmarineTrait for Submarine {
    fn travel_part_one(&mut self, direction: &Movement) -> Result<(), AocError> {
        match direction {
            Movement::Forward(val) => self.pos_x += *val,
            Movement::Up(val) => self.pos_y.my_checked_sub(*val)?,
            Movement::Down(val) => self.pos_y.my_checked_add(*val)?,
        };
        Ok(())
    }

    fn travel_part_two(&mut self, direction: &Movement) -> Result<(), AocError> {
        match direction {
            Movement::Forward(val) => {
                self.pos_x += *val;
                self.pos_y += self.aim * *val;
            }
            Movement::Up(val) => self.aim.my_checked_sub(*val)?,
            Movement::Down(val) => self.aim.my_checked_add(*val)?,
        };
        Ok(())
    }
}

impl Submarine {
    pub fn get_x(&self) -> u32 {
        self.pos_x
    }

    pub fn get_y(&self) -> u32 {
        self.pos_y
    }
}

#[derive(Debug)]
pub enum Movement {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Movement {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let vec = input.split_whitespace().collect::<Vec<&str>>();

        let magnitude = vec
            .get(1)
            .ok_or_else(|| AocError::InputError(input.to_string()))?
            .parse::<u32>()?;

        #[allow(clippy::useless_asref)] // as_ref() not actually useless here
        match vec
            .get(0)
            .ok_or_else(|| AocError::InputError(input.to_string()))?
            .as_ref()
        {
            "forward" => Ok(Movement::Forward(magnitude)),
            "up" => Ok(Movement::Up(magnitude)),
            "down" => Ok(Movement::Down(magnitude)),
            _ => Err(AocError::InputError(input.to_string())),
        }
    }
}
