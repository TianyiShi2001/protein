use crate::AminoAcid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SEQRES records contain a listing of the consecutive chemical components covalently linked
/// in a linear fashion to form a polymer. The chemical components included in this listing may
/// be standard or modified amino acid and nucleic acid residues. It may also include other residues
/// that are linked to the standard backbone in the polymer. Chemical components or groups covalently
/// linked to side-chains (in peptides) or sugars and/or bases (in nucleic acid polymers) will not be
/// listed here.
///
/// # Record Format
///
/// | COLUMNS | DATA TYPE    | FIELD    | DEFINITION                                                                                                                        |
/// | ------- | ------------ | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
/// | 1 -  6  | Record name  | "SEQRES" |                                                                                                                                   |
/// | 8 - 10  | Integer      | serNum   | Serial number of the SEQRES record for the current chain. Starts at 1 and increments by one each line. Reset to 1 for each chain. |
/// | 12      | Character    | chainID  | Chain identifier. This may be any single legal character, including a blank which is used if there is only one chain.             |
/// | 14 - 17 | Integer      | numRes   | Number of residues in the chain. This  value is repeated on every record.                                                         |
/// | 20 - 22 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 24 - 26 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 28 - 30 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 32 - 34 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 36 - 38 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 40 - 42 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 44 - 46 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 48 - 50 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 52 - 54 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 56 - 58 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 60 - 62 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 64 - 66 | Residue name | resName  | Residue name.                                                                                                                     |
/// | 68 - 70 | Residue name | resName  | Residue name.                                                                                                                     |
pub type SeqRes = Vec<(char, Vec<AminoAcid>)>;

type Chain = char;
type SequenceNumber = u32;
type Position = (Chain, SequenceNumber);

/// The [MODRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
/// record provides descriptions of modifications (e.g., chemical or post-translational) to protein
/// and nucleic acid residues. Included are correlations between residue names given in a PDB entry
/// and standard residues.
///
/// # Record Format
///
/// | COLUMNS | DATA TYPE    | FIELD    | DEFINITION                               |
/// | ------- | ------------ | -------- | ---------------------------------------- |
/// | 1 -  6  | Record name  | "MODRES" |                                          |
/// | 8 - 11  | IDcode       | idCode   | ID code of this entry.                   |
/// | 13 - 15 | Residue name | resName  | Residue name used in this entry.         |
/// | 17      | Character    | chainID  | Chain identifier.                        |
/// | 19 - 22 | Integer      | seqNum   | Sequence number.                         |
/// | 23      | AChar        | iCode    | Insertion code.                          |
/// | 25 - 27 | Residue name | stdRes   | Standard residue name.                   |
/// | 30 - 70 | String       | comment  | Description of the residue modification. |
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomAminoAcid {
    pub standard_res: AminoAcid,
    pub description: String,
    pub occurence: Vec<Position>,
}

pub type Modres = HashMap<String, CustomAminoAcid>;
