use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Element {
    H,
    C,
    O,
    N,
    P,
    S,
    Na,
    Mg,
    Cl,
    K,
    Ca,
    Fe,
    Mn,
    Co,
    Cr,
    I,
    Zn,
    Cu,
    F,
    Al,
    Se,
    V,
}

impl FromStr for Element {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "H" => Ok(Element::H),
            "C" => Ok(Element::C),
            "O" => Ok(Element::O),
            "N" => Ok(Element::N),
            "P" => Ok(Element::P),
            "S" => Ok(Element::S),
            "Na" => Ok(Element::Na),
            "Mg" => Ok(Element::Mg),
            "Cl" => Ok(Element::Cl),
            "K" => Ok(Element::K),
            "Ca" => Ok(Element::Ca),
            "Fe" => Ok(Element::Fe),
            "Mn" => Ok(Element::Mn),
            "Co" => Ok(Element::Co),
            "Cr" => Ok(Element::Cr),
            "I" => Ok(Element::I),
            "Zn" => Ok(Element::Zn),
            "Cu" => Ok(Element::Cu),
            "F" => Ok(Element::F),
            "Al" => Ok(Element::Al),
            "Se" => Ok(Element::Se),
            "V" => Ok(Element::V),
            _ => Err(format!("Unknown atom name {}", inp)),
        }
    }
}
