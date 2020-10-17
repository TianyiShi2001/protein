use crate::types::{Anisou, Atom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub atoms: Vec<Atom>,
    pub anisou: Vec<Anisou>,
}
