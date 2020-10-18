use super::AtomSerial;
use crate::types::{Anisou, Atom, Helix, Sheet};
use serde::{Deserialize, Serialize};
pub type Connect = [AtomSerial; 2];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub atoms: Vec<Atom>,
    pub anisou: Vec<Anisou>,
    pub connect: Vec<Connect>,
    pub sheets: Vec<Sheet>,
    pub helices: Vec<Helix>,
}
