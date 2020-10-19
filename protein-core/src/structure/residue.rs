use super::Element;
use serde::Serialize;
use strum::EnumString;

#[derive(Debug, Clone, Serialize)]
pub enum Residue {
    AminoAcid(AminoAcid),
    Nucleotide(Nucleotide),
    Water,
    Metal(Element),
    Molecule(Molecule),
    Other(String),
    UnknownAtomOrIon,
    UnknownLigand,
}

pub enum StandardResidue {
    AminoAcid(StandardAminoAcid),
    Nucleotide(StandardNucleotide),
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct Molecule {
    code: String,
    description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Nucleotide {
    Standard(StandardNucleotide),
    Modified(String),
    Unknown, // represented as "N" in PDB files
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum AminoAcid {
    Standard(StandardAminoAcid),
    Modified(String),
    Unknown, // represented as "UNK" in PDB files
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
