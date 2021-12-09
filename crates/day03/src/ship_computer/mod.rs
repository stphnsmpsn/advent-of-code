use crate::repo::Repo;
use crate::ship_computer::diagnostic::DiagnosticEntry;
use crate::util::{common_char_in_position, Commonality, ONE, ZERO};
use common::error::AocError;

pub mod diagnostic;

const BINARY_RADIX: u32 = 2;

pub struct ShipComputer {
    // keeping this allows us to avoid iterating over the entire Vec<Diagnostic> to get the most
    // common value in this position. This lets us calculate part 1 much faster.
    bit_accumulator: Vec<(u32, u32)>,
    // added in part two. I don't like storing this in-memory. Have abstracted as I would opt to
    // use a db so as not to exhause memory in exceptionally large datasets
    diagnostic_repo: Box<dyn Repo>,
}

impl ShipComputer {
    pub fn new(diagnostic_repo: Box<dyn Repo>) -> Self {
        Self {
            bit_accumulator: vec![],
            diagnostic_repo,
        }
    }

    pub fn add_diagnostic_entry(
        &mut self,
        diagnostic_entry: DiagnosticEntry,
    ) -> Result<(), AocError> {
        if self.bit_accumulator.is_empty() {
            self.bit_accumulator = vec![(0, 0); diagnostic_entry.len()];
        }

        for (i, elem) in self.bit_accumulator.iter_mut().enumerate() {
            match diagnostic_entry.char_at_bit(i)? {
                ZERO => elem.0 += 1,
                ONE => elem.1 += 1,
                _ => {
                    return Err(AocError::InputError(format!(
                        "Invalid Binary Character in: {:?}",
                        diagnostic_entry
                    )))
                }
            }
        }

        self.diagnostic_repo.store(diagnostic_entry)?;

        Ok(())
    }

    pub fn calculate_power_consumption(&self) -> Result<u32, AocError> {
        Ok(self.delta()? * self.gamma()?)
    }

    pub fn calculate_life_support(&self) -> Result<u32, AocError> {
        Ok(self.o2_generator_rating()? * self.co2_scrubber_rating()?)
    }

    fn gamma(&self) -> Result<u32, AocError> {
        Ok(u32::from_str_radix(
            self.bit_accumulator
                .iter()
                .map(|a| if a.0 > a.1 { ZERO } else { ONE })
                .collect::<String>()
                .as_str(),
            BINARY_RADIX,
        )?)
    }

    fn delta(&self) -> Result<u32, AocError> {
        Ok(u32::from_str_radix(
            self.bit_accumulator
                .iter()
                .map(|a| if a.0 < a.1 { ZERO } else { ONE })
                .collect::<String>()
                .as_str(),
            BINARY_RADIX,
        )?)
    }

    fn co2_scrubber_rating(&self) -> Result<u32, AocError> {
        let num_positions = self.bit_accumulator.len();
        let mut remaining = self.diagnostic_repo.retrieve(0, u32::MAX)?;

        remaining = filter_remaining(remaining, num_positions, Commonality::Least)?;

        Ok(u32::from_str_radix(
            remaining[0].bits.iter().collect::<String>().as_str(),
            2,
        )?)
    }

    fn o2_generator_rating(&self) -> Result<u32, AocError> {
        let num_positions = self.bit_accumulator.len();
        let mut remaining = self.diagnostic_repo.retrieve(0, u32::MAX)?;

        remaining = filter_remaining(remaining, num_positions, Commonality::Most)?;

        Ok(u32::from_str_radix(
            remaining[0].bits.iter().collect::<String>().as_str(),
            2,
        )?)
    }
}

fn filter_remaining(
    mut remaining: Vec<DiagnosticEntry>,
    num_positions: usize,
    commonality: Commonality,
) -> Result<Vec<DiagnosticEntry>, AocError> {
    for i in 0..num_positions {
        if remaining.len() == 1 {
            break;
        }

        remaining = remaining
            .iter()
            .cloned()
            .filter_map(|e| {
                let x = common_char_in_position(&remaining, i, commonality).ok()?;
                match e.char_at_bit(i).ok()? == x {
                    true => Some(e),
                    false => None,
                }
            })
            .collect();
    }

    Ok(remaining)
}
