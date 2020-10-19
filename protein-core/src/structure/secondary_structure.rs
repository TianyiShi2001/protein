use super::serial::{ResidueSerial, SecondaryStructureSerial};
use crate::AminoAcid;
use crate::AtomName;
use serde::Serialize;

/// # Overview
///
/// HELIX records are used to identify the position of helices in the molecule. Helices are named, numbered, and classified by type. The residues where the helix begins and ends are noted, as well as the total length.
///
/// # Record Format
///
///|COLUMNS  |      DATA  TYPE    | FIELD        | DEFINITION
///|---------|--------------------|--------------|----------------------------------------
///| 1 -  6  |      Record name   | "HELIX "     |                                         
///| 8 - 10  |      Integer       | serNum       | Serial number of the helix. This starts
///|         |                    |              | at 1  and increases incrementally.
///|12 - 14  |      LString(3)    | helixID      | Helix  identifier. In addition to a serial
///|         |                    |              | number, each helix is given an
///|         |                    |              | alphanumeric character helix identifier.
///|16 - 18  |      Residue name  | initResName  | Name of the initial residue.
///|20       |      Character     | initChainID  | Chain identifier for the chain containing
///|         |                    |              | this  helix.
///|22 - 25  |      Integer       | initSeqNum   | Sequence number of the initial residue.
///|26       |      AChar         | initICode    | Insertion code of the initial residue.
///|28 - 30  |      Residue  name | endResName   | Name of the terminal residue of the helix.
///|32       |      Character     | endChainID   | Chain identifier for the chain containing
///|         |                    |              | this  helix.
///|34 - 37  |      Integer       | endSeqNum    | Sequence number of the terminal residue.
///|38       |      AChar         | endICode     | Insertion code of the terminal residue.
///|39 - 40  |      Integer       | helixClass   | Helix class (see below).
///|41 - 70  |      String        | comment      | Comment about this helix.
///|72 - 76  |      Integer       | length       | Length of this helix.
///
/// # Details
///
/// Additional HELIX records with different serial numbers and identifiers occur if more than one helix is present.
/// The initial residue of the helix is the N-terminal residue.
/// Helices are classified as follows:
///
/// |                                |     CLASS NUMBER             |           
/// |TYPE OF  HELIX                  |   (COLUMNS 39 - 40)          |              
/// |--------------------------------|------------------------------|
/// |Right-handed alpha (default)    |            1                 |       
/// |Right-handed omega              |            2                 |       
/// |Right-handed pi                 |            3                 |       
/// |Right-handed gamma              |            4                 |       
/// |Right-handed 3 - 10             |            5                 |       
/// |Left-handed alpha               |            6                 |       
/// |Left-handed omega               |            7                 |       
/// |Left-handed gamma               |            8                 |       
/// |2 - 7 ribbon/helix              |            9                 |       
/// |Polyproline                     |           10                 |       
#[derive(Debug, Clone, Serialize)]
pub struct Helix {
    // pub serial: SecondaryStructureSerial, // * implied
    pub id: String,
    pub class: HelixClass,
    pub start: (char, ResidueSerial),
    //start_res: AminoAcid,
    pub end: (char, ResidueSerial),
    //end_res: AminoAcid,
    pub comment: String,
    // length: usize, // * implied
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum HelixClass {
    RightHandedAlpha,
    RightHandedOmega,
    RightHandedPi,
    RightHandedGamma,
    RightHanded310,
    LeftHandedAlpha,
    LeftHandedOmega,
    LeftHandedGamma,
    TwoSevenRibbonHelix,
    Polyproline,
    Unknown,
}

impl Default for HelixClass {
    fn default() -> Self {
        Self::Unknown
    }
}

/// # Fields
///
///
/// # PDB Specification
///
/// ## Overview
///
/// SHEET records are used to identify the position of sheets in the molecule. Sheets are both named and numbered. The residues where the sheet begins and ends are noted.
///
/// ## Record Format
///
/// | COLUMNS | DATA  TYPE   | FIELD       | DEFINITION                                        |
/// | ------- | ------------ | ----------- | ------------------------------------------------- |
/// | 1 -  6  | Record name  | "SHEET "    |                                                   |
/// | 8 - 10  | Integer      | strand      | Strand  number which starts at 1 for each         |
/// |         |              |             | strand within a sheet and increases by one.       |
/// | 12 - 14 | LString(3)   | sheetID     | Sheet  identifier.                                |
/// | 15 - 16 | Integer      | numStrands  | Number  of strands in sheet.                      |
/// | 18 - 20 | Residue name | initResName | Residue  name of initial residue.                 |
/// | 22      | Character    | initChainID | Chain identifier of initial residue               |
/// |         |              |             | in strand.                                        |
/// | 23 - 26 | Integer      | initSeqNum  | Sequence number of initial residue                |
/// |         |              |             | in strand.                                        |
/// | 27      | AChar        | initICode   | Insertion code of initial residue                 |
/// |         |              |             | in  strand.                                       |
/// | 29 - 31 | Residue name | endResName  | Residue name of terminal residue.                 |
/// | 33      | Character    | endChainID  | Chain identifier of terminal residue.             |
/// | 34 - 37 | Integer      | endSeqNum   | Sequence number of terminal residue.              |
/// | 38      | AChar        | endICode    | Insertion code of terminal residue.               |
/// | 39 - 40 | Integer      | sense       | Sense of strand with respect to previous          |
/// |         |              |             | strand in the sheet. 0 if first strand,           |
/// |         |              |             | 1 if  parallel,and -1 if anti-parallel.           |
/// | 42 - 45 | Atom         | curAtom     | Registration.  Atom name in current strand.       |
/// | 46 - 48 | Residue name | curResName  | Registration.  Residue name in current strand     |
/// | 50      | Character    | curChainId  | Registration. Chain identifier in                 |
/// |         |              |             | current strand.                                   |
/// | 51 - 54 | Integer      | curResSeq   | Registration.  Residue sequence number            |
/// |         |              |             | in current strand.                                |
/// | 55      | AChar        | curICode    | Registration. Insertion code in                   |
/// |         |              |             | current strand.                                   |
/// | 57 - 60 | Atom         | prevAtom    | Registration.  Atom name in previous strand.      |
/// | 61 - 63 | Residue name | prevResName | Registration.  Residue name in                    |
/// |         |              |             | previous strand.                                  |
/// | 65      | Character    | prevChainId | Registration.  Chain identifier in                |
/// |         |              |             | previous  strand.                                 |
/// | 66 - 69 | Integer      | prevResSeq  | Registration. Residue sequence number             |
/// |         |              |             | in previous strand.                               |
/// | 70      | AChar        | prevICode   | Registration.  Insertion code in previous strand. |
///
/// ## Details
///
/// - The initial residue for a strand is its N-terminus. Strand registration information is provided in columns 39 - 70. Strands are listed starting with one edge of the sheet and continuing to the spatially adjacent strand.
/// - The sense in columns 39 - 40 indicates whether strand n is parallel (sense = 1) or anti-parallel (sense = -1) to strand n-1. Sense is equal to zero (0) for the first strand of a sheet.
/// - The registration (columns 42 - 70) of strand n to strand n-1 may be specified by one hydrogen bond between each such pair of strands. This is done by providing the hydrogen bonding between the current and previous strands. No register information should be provided for the first strand.
/// - Split strands, or strands with two or more runs of residues from discontinuous parts of the amino acid sequence, are explicitly listed. Detail description can be included in the REMARK 700 .
#[derive(Serialize, Debug, Default, Clone)]
pub struct Sheet {
    pub id: String,
    pub strands: Vec<Strand>,
    pub registration: Vec<Registration>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Strand {
    //pub serial: SsSerial, // * implied
    //pub nstrand: u8, // * implied
    pub start: (char, ResidueSerial),
    pub end: (char, ResidueSerial),
    //start_res: AminoAcid,
    //end_res: AminoAcid,
    pub sense: Sense,
}

/// # Fields
///
/// `cur` and `prev` are tuples containing 1. the element (N or O) 2. the chain identifier and 3. sequence number
#[derive(Serialize, Debug, Clone)]
pub struct Registration {
    pub curr: (AtomName, char, ResidueSerial),
    pub prev: (AtomName, char, ResidueSerial),
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Sense {
    Parallel,
    Antiparallel,
    Unknown,
}
