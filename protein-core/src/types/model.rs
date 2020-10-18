use super::AtomId;
use crate::types::{Anisou, Atom};
use serde::{Deserialize, Serialize};
pub type Connect = [AtomId; 2];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub atoms: Vec<Atom>,
    pub anisou: Vec<Anisou>,
    pub connect: Vec<Connect>,
}
