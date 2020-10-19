use serde::Serialize;
use strum::EnumString;

#[derive(Debug, Clone, Serialize)]
pub enum Residue {
    AminoAcid(AminoAcid),
    Nucleotide(Nucleotide),
    Water,
    Molecule(String),
    Unknown(String),
}

pub enum StandardResidue {
    AminoAcid(StandardAminoAcid),
    Nucleotide(StandardNucleotide),
    Unknown,
}

// impl Residue {
//     pub fn from_bytes_uppercase_fixed3(inp: &[u8]) -> Self {
//         if let Some(aa)
//     }
// }

pub struct Molecule {
    code: String,
    description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Nucleotide {
    Standard(StandardNucleotide),
    Modified(String),
}

impl Nucleotide {
    pub fn from_uppercase(inp: &str) -> Self {
        if let Some(aa) = StandardNucleotide::from_uppercase(inp) {
            Self::Standard(aa)
        } else {
            Self::Modified(inp.to_owned())
        }
    }
    pub fn from_bytes_uppercase_fixed3(inp: &[u8]) -> Self {
        if let Some(nuc) = StandardNucleotide::from_bytes_uppercase_fixed3(inp) {
            Self::Standard(nuc)
        } else {
            Self::Modified(unsafe { String::from_utf8_unchecked(inp.to_owned()) })
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, EnumString)]
pub enum StandardNucleotide {
    A,
    C,
    G,
    U,
    DA,
    DC,
    DG,
    DT,
}

impl StandardNucleotide {
    pub fn from_uppercase(inp: &str) -> Option<Self> {
        Self::from_bytes_uppercase(inp.as_bytes())
    }
    pub fn from_bytes_uppercase(inp: &[u8]) -> Option<Self> {
        match inp {
            b"A" => Some(Self::A),
            b"C" => Some(Self::C),
            b"G" => Some(Self::G),
            b"U" => Some(Self::U),
            b"DA" => Some(Self::DA),
            b"DC" => Some(Self::DC),
            b"DG" => Some(Self::DG),
            b"DT" => Some(Self::DT),
            _ => None,
        }
    }
    pub fn from_uppercase_fixed3(inp: &str) -> Option<Self> {
        Self::from_bytes_uppercase_fixed3(inp.as_bytes())
    }
    pub fn from_bytes_uppercase_fixed3(inp: &[u8]) -> Option<Self> {
        match inp {
            b"  A" => Some(Self::A),
            b"  C" => Some(Self::C),
            b"  G" => Some(Self::G),
            b"  U" => Some(Self::U),
            b" DA" => Some(Self::DA),
            b" DC" => Some(Self::DC),
            b" DG" => Some(Self::DG),
            b" DT" => Some(Self::DT),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum AminoAcid {
    Standard(StandardAminoAcid),
    Modified(String),
}

impl AminoAcid {
    pub fn from_uppercase(inp: &str) -> Self {
        if let Some(aa) = StandardAminoAcid::from_uppercase(inp) {
            Self::Standard(aa)
        } else {
            Self::Modified(inp.to_owned())
        }
    }
    pub fn from_bytes_uppercase(inp: &[u8]) -> Self {
        if let Some(aa) = StandardAminoAcid::from_bytes_uppercase(inp) {
            Self::Standard(aa)
        } else {
            Self::Modified(unsafe { String::from_utf8_unchecked(inp.to_owned()) })
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum StandardAminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
    Mse,
    Pyl, // https://www.wwpdb.org/news/news?year=2014#5764490799cccf749a90cddf
    Sec, // https://www.wwpdb.org/news/news?year=2014#5764490799cccf749a90cddf
}

impl StandardAminoAcid {
    pub fn from_uppercase(inp: &str) -> Option<Self> {
        Self::from_bytes_uppercase(inp.as_bytes())
    }
    pub fn from_bytes_uppercase(inp: &[u8]) -> Option<Self> {
        match inp {
            b"ALA" => Some(Self::Ala),
            b"ARG" => Some(Self::Arg),
            b"ASN" => Some(Self::Asn),
            b"ASP" => Some(Self::Asp),
            b"CYS" => Some(Self::Cys),
            b"GLN" => Some(Self::Gln),
            b"GLU" => Some(Self::Glu),
            b"GLY" => Some(Self::Gly),
            b"HIS" => Some(Self::His),
            b"ILE" => Some(Self::Ile),
            b"LEU" => Some(Self::Leu),
            b"LYS" => Some(Self::Lys),
            b"MET" => Some(Self::Met),
            b"PHE" => Some(Self::Phe),
            b"PRO" => Some(Self::Pro),
            b"SER" => Some(Self::Ser),
            b"THR" => Some(Self::Thr),
            b"TRP" => Some(Self::Trp),
            b"TYR" => Some(Self::Tyr),
            b"VAL" => Some(Self::Val),
            b"PYL" => Some(Self::Pyl),
            b"SEC" => Some(Self::Sec),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct ModifiedAminoAcid {
    pub standard: StandardAminoAcid,
    pub description: String,
}

pub trait Monomer {}

impl Monomer for AminoAcid {}

impl Monomer for Nucleotide {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct ModifiedNucleotide {
    pub standard: StandardNucleotide,
    pub description: String,
}

pub type ModifiedNucleotideList = Vec<ModifiedNucleotide>;
