use crate::ship_computer::diagnostic::DiagnosticEntry;
use common::error::AocError;

pub mod in_memory_repo;

pub trait Repo {
    fn store(&mut self, diagnostic: DiagnosticEntry) -> Result<(), AocError>;
    fn retrieve(&self, cursor: usize, limit: u32) -> Result<Vec<DiagnosticEntry>, AocError>;
    fn len(&self) -> Result<usize, AocError>;
}
