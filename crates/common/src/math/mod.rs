use crate::error::AocError;

pub trait MyCheckedOps {
    fn my_checked_add(&mut self, rhs: u32) -> Result<(), AocError>;
    fn my_checked_sub(&mut self, rhs: u32) -> Result<(), AocError>;
    fn my_checked_mul(&mut self, rhs: u32) -> Result<(), AocError>;
}

enum Operation {
    Add,
    Subtract,
    Multiply,
}

impl MyCheckedOps for u32 {
    fn my_checked_add(&mut self, rhs: u32) -> Result<(), AocError> {
        do_work(self, rhs, Operation::Add)
    }

    fn my_checked_sub(&mut self, rhs: u32) -> Result<(), AocError> {
        do_work(self, rhs, Operation::Subtract)
    }

    fn my_checked_mul(&mut self, rhs: u32) -> Result<(), AocError> {
        do_work(self, rhs, Operation::Multiply)
    }
}

fn do_work(me: &mut u32, rhs: u32, op: Operation) -> Result<(), AocError> {
    let res = match op {
        Operation::Add => me.checked_add(rhs),
        Operation::Subtract => me.checked_sub(rhs),
        Operation::Multiply => me.checked_mul(rhs),
    };

    match res {
        Some(res) => {
            *me = res;
            Ok(())
        }
        None => {
            return Err(AocError::OverflowError(format!(
                "Overflow calculating: {} * {}",
                me, rhs
            )))
        }
    }
}
