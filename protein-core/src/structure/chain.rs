use super::Monomer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Chain<T: Monomer> {
    pub id: char,
    pub seq: Vec<T>,
}
