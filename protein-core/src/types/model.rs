use super::AtomSerial;
use crate::types::{
    AminoAcid, Anisou, Atom, Chain, Helix, ModifiedAminoAcid, ModifiedNucleotide, Nucleotide, Sheet,
};
use serde::Serialize;
use std::collections::HashMap;

pub type Connect = [AtomSerial; 2];

#[derive(Debug, Clone, Serialize, Default)]
pub struct Structure {
    pub chains_aa: Vec<Chain<AminoAcid>>,
    pub chains_nuc: Vec<Chain<Nucleotide>>,
    pub helices: Vec<Helix>,
    pub sheets: Vec<Sheet>,
    pub modified_aa: HashMap<String, ModifiedAminoAcid>,
    pub modified_nuc: HashMap<String, ModifiedNucleotide>,
    pub connect: Vec<Connect>,
    pub models: Vec<Model>,
    // pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Model {
    pub atoms: Vec<Atom>,
    pub anisou: Vec<Anisou>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Metadata {
    // pub header: Header,
// pub title: Title,
// pub authors: Authors,
// pub experimental_techniques: ExperimentalTechniques,
// pub cryst1: Cryst1,
}
