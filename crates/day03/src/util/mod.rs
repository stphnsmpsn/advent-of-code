use crate::ship_computer::diagnostic::DiagnosticEntry;
use common::error::AocError;

pub const ZERO: char = '0';
pub const ONE: char = '1';

#[derive(Clone, Copy)]
pub enum Commonality {
    Least,
    Most,
}

pub fn common_char_in_position(
    vec: &[DiagnosticEntry],
    pos: usize,
    commonality: Commonality,
) -> Result<char, AocError> {
    let num_bits = vec.get(0).unwrap().len();
    let mut bit_accumulator: Vec<(u32, u32)> = vec![(0, 0); num_bits];

    for v in vec {
        for (i, (zeros, ones)) in bit_accumulator.iter_mut().enumerate() {
            match v.char_at_bit(i)? {
                ZERO => *zeros += 1,
                ONE => *ones += 1,
                _ => return Err(AocError::GenericError("".to_string())),
            }
        }
    }

    Ok(match commonality {
        Commonality::Least => {
            if bit_accumulator[pos].0 <= bit_accumulator[pos].1 {
                '0'
            } else {
                '1'
            }
        }
        Commonality::Most => {
            if bit_accumulator[pos].1 >= bit_accumulator[pos].0 {
                '1'
            } else {
                '0'
            }
        }
    })
}
