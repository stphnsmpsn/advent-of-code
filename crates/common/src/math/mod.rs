use crate::error::AocError;

pub trait MyCheckedOps {
    fn my_checked_add(&mut self, rhs: u32) -> Result<(), AocError>;
    fn my_checked_sub(&mut self, rhs: u32) -> Result<(), AocError>;
    fn my_checked_mul(&mut self, rhs: u32) -> Result<(), AocError>;
}

impl MyCheckedOps for u32 {
    fn my_checked_add(&mut self, rhs: u32) -> Result<(), AocError> {
        match self.checked_add(rhs) {
            Some(res) => {
                *self = res;
                Ok(())
            }
            None => {
                return Err(AocError::OverflowError(format!(
                    "Overflow calculating: {} + {}",
                    self, rhs
                )))
            }
        }
    }

    fn my_checked_sub(&mut self, rhs: u32) -> Result<(), AocError> {
        match self.checked_sub(rhs) {
            Some(res) => {
                *self = res;
                Ok(())
            }
            None => {
                return Err(AocError::OverflowError(format!(
                    "Overflow calculating: {} - {}",
                    self, rhs
                )))
            }
        }
    }

    fn my_checked_mul(&mut self, rhs: u32) -> Result<(), AocError> {
        match self.checked_mul(rhs) {
            Some(res) => {
                *self = res;
                Ok(())
            }
            None => {
                return Err(AocError::OverflowError(format!(
                    "Overflow calculating: {} * {}",
                    self, rhs
                )))
            }
        }
    }
}
