use crate::types::{AminoAcid, Element};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::AtomSerial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atom {
    pub id: AtomSerial,
    pub name: AtomName,
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
pub enum AtomName {
    N,
    CA,
    C,
    O,
    Other(String), // TODO: a 'lossy' version?
}

impl FromStr for AtomName {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "C" => Ok(AtomName::C),
            "CA" => Ok(AtomName::CA),
            "O" => Ok(AtomName::O),
            "N" => Ok(AtomName::N),
            _ => Ok(AtomName::Other(inp.to_owned())),
        }
    }
}
