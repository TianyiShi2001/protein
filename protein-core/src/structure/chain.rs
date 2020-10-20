use super::Monomer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Chain<T: Monomer> {
    pub id: char,
    pub seq: Vec<T>,
}

impl<T: Monomer> Chain<T> {
    pub fn len(&self) -> usize {
        self.seq.len()
    }
    pub fn is_empty(&self) -> bool {
        self.seq.is_empty()
    }
}
