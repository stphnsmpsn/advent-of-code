use crate::util::{ONE, ZERO};
use common::error::AocError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DiagnosticEntry {
    pub bits: Vec<char>,
}

impl DiagnosticEntry {
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn char_at_bit(&self, pos: usize) -> Result<char, AocError> {
        Ok(*self
            .bits
            .get(pos)
            .ok_or_else(|| AocError::GenericError("".to_string()))?)
    }
}

impl FromStr for DiagnosticEntry {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(DiagnosticEntry {
            bits: input
                .chars()
                .map(|c| match c {
                    ZERO | ONE => Ok(c),
                    _ => Err(AocError::InputError(input.to_string())),
                })
                .collect::<Result<Vec<char>, AocError>>()?,
        })
    }
}
