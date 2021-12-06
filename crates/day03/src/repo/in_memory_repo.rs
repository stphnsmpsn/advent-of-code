use crate::repo::Repo;
use crate::ship_computer::diagnostic::DiagnosticEntry;
use common::error::AocError;

#[derive(Default)]
pub struct InMemoryRepo {
    diagnostics: Vec<DiagnosticEntry>,
}

impl Repo for InMemoryRepo {
    fn store(&mut self, diagnostic: DiagnosticEntry) -> Result<(), AocError> {
        self.diagnostics.push(diagnostic);
        Ok(())
    }

    fn retrieve(&self, cursor: usize, limit: u32) -> Result<Vec<DiagnosticEntry>, AocError> {
        Ok(self
            .diagnostics
            .iter()
            .skip(cursor)
            .take(limit as usize)
            .cloned()
            .collect::<Vec<DiagnosticEntry>>())
    }

    fn len(&self) -> Result<usize, AocError> {
        Ok(self.diagnostics.len())
    }
}
