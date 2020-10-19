use super::{Anisou, Atom};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Model {
    pub atoms: Vec<Atom>,
    pub anisou: Vec<Anisou>,
}
