//! Structure

// The major complexity arises from non-standard/modified amino acid and nucleotide residues and hetero-residues (non-amino-acid and non-nucleotide  molecules).

pub mod anisou;
pub use anisou::Anisou;
pub mod atom;
pub use atom::{Atom, AtomName}; // NucleotideAtomName, AminoAcidAtomName};
pub mod element;
pub use element::Element;
pub mod model;
pub use model::Model;
pub mod secondary_structure;
pub use secondary_structure::{Helix, HelixClass, Registration, Sense, Sheet, Strand};
pub mod serial;
pub use serial::*;
pub mod residue;
pub use residue::{
    AminoAcid, ModifiedAminoAcid, ModifiedNucleotide, Molecule, Monomer, Nucleotide, Residue,
    StandardAminoAcid, StandardNucleotide,
};
pub mod chain;
pub use chain::Chain;

pub type Connect = [AtomSerial; 2];

use serde::Serialize;
use std::collections::HashMap;

use crate::metadata::Metadata;

#[derive(Debug, Clone, Serialize)]
pub struct Ssbond {
    pub a: (char, ResidueSerial),
    pub b: (char, ResidueSerial),
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Structure {
    pub chains_aa: Vec<Chain<AminoAcid>>,
    pub chains_nuc: Vec<Chain<Nucleotide>>,
    pub helices: Vec<Helix>,
    pub sheets: Vec<Sheet>,
    pub ssbonds: Vec<Ssbond>,
    pub modified_aa: HashMap<String, ModifiedAminoAcid>,
    pub modified_nuc: HashMap<String, ModifiedNucleotide>,
    pub connect: Vec<Connect>,
    // pub link: Vec<Link>
    pub models: Vec<Model>,
    pub metadata: Option<Metadata>,
}
