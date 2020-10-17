use serde::{Deserialize, Serialize};
pub mod title_section;
pub use title_section::*;
pub mod crystallography;
pub use crystallography::*;
pub mod primary_structure;
pub use primary_structure::*;

use crate::Model;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pdb {
    pub header: Header,
    pub title: Title,
    pub authors: Authors,
    pub experimental_techniques: ExperimentalTechniques,
    pub cryst1: Cryst1,
    pub modres: Modres,
    pub seqres: SeqRes,
    pub models: Vec<Model>,
}
