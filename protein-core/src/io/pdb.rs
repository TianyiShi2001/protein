use serde::Serialize;
pub mod title_section;
pub use title_section::*;
pub mod crystallography;
pub use crystallography::*;
pub mod primary_structure;
pub use primary_structure::*;

use crate::Structure;

// #[derive(Debug, Clone, Default, Serialize)]
// pub struct Pdb {
//     pub header: Header,
//     pub title: Title,
//     pub authors: Authors,
//     pub experimental_techniques: ExperimentalTechniques,
//     pub cryst1: Cryst1,
//     // pub modres: Modres,
//     // pub seqres: SeqRes,
//     pub structure: Structure,
// }
