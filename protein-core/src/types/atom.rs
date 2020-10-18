use crate::types::{AminoAcid, Element};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::AtomId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atom {
    pub id: AtomId,
    pub name: AminoAcidAtomName,
    pub id1: char,
    pub residue: AminoAcid,
    pub chain: char,
    pub sequence_number: u32,
    pub insertion_code: char,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub occupancy: f32,
    pub temperature_factor: f32,
    pub element: Element,
    pub charge: i8,
    pub hetatom: bool,
}

pub struct GenericAtomParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AminoAcidAtomName {
    N,
    CA,
    C,
    O,
    Other(String), // TODO: a 'lossy' version?
}

impl FromStr for AminoAcidAtomName {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "C" => Ok(AminoAcidAtomName::C),
            "CA" => Ok(AminoAcidAtomName::CA),
            "O" => Ok(AminoAcidAtomName::O),
            "N" => Ok(AminoAcidAtomName::N),
            _ => Ok(AminoAcidAtomName::Other(inp.to_owned())),
        }
    }
}
