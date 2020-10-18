pub type AtomId = usize;

pub mod aa;
pub use aa::{AminoAcid, NonstandardAminoAcid};
pub mod anisou;
pub use anisou::Anisou;
pub mod atom;
pub use atom::{AminoAcidAtomName, Atom};
pub mod element;
pub use element::Element;
pub mod model;
pub use model::{Connect, Model};
