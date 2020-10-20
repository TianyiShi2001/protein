use super::AtomSerial;
use super::{Element, Residue};
use serde::Serialize;
use std::str::FromStr;

/// Represents an atom in a structural model.
///
/// # Field
///
/// - `id`: Unique identifier. No two atoms. No two `Atom`s in the same `Model` can have the same `id`.
/// - `name`: Atom name. This represents the exact "role" of an atom in a given residue.
/// - `residue`: The [`Residue`] this atom belongs to. This can be an amino acid, a nucleotide, water, or a custom molecule.
/// - `chain`: The chain identifier
/// - `sequence_number`: The position of this atom in the chain
/// - `coord`: Cartesian coordinates `x, y, z`
/// - `occupancy`:
/// - `temperature_factor`:
/// - `element`: The [`Element`]
/// - `chaege`: The charge on the atom
///
/// [`Residue`]: ../residue/struct.Residue.html
/// [`Element`]: ../element/enum.Element.html
#[derive(Debug, Clone, Serialize)]
pub struct Atom {
    pub id: AtomSerial,
    pub name: AtomName,
    pub id1: char,
    pub residue: Residue,
    pub chain: char,
    pub sequence_number: u32,
    pub insertion_code: char,
    pub coord: [f32; 3],
    pub occupancy: f32,
    pub temperature_factor: f32,
    pub element: Element,
    pub charge: i8,
    // pub is_hetatom: bool, // ! implied
}

pub struct GenericAtomParser;

/// Internally, an `AtomName` is represented as an array of 4 bytes.
///
/// For convinience, `FromStr` is implemented for `AtomName`, so you can:
///
/// ```
/// use std::str::FromStr;
/// use protein_core::structure::AtomName;
///
/// let atom = AtomName::from_str("O2").unwrap();
/// assert_eq!(atom, AtomName([b'O', b'2', b' ', b' ']));
/// ```
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AtomName(pub [u8; 4]);

impl AtomName {
    pub(crate) fn is_n(&self) -> bool {
        &self.0 == b"N   "
    }
    pub(crate) fn is_c(&self) -> bool {
        &self.0 == b"C   "
    }
    pub(crate) fn is_ca(&self) -> bool {
        &self.0 == b"CA  "
    }
    pub(crate) fn is_o(&self) -> bool {
        &self.0 == b"O   "
    }
}

#[derive(Debug)]
pub enum ParseAtomNameError {
    LengthGreaterThan4,
    LengthZero,
}

impl FromStr for AtomName {
    type Err = ParseAtomNameError;
    fn from_str(s: &str) -> Result<Self, ParseAtomNameError> {
        let s = s.as_bytes();
        match s.len() {
            1usize => Ok(AtomName([s[0], b' ', b' ', b' '])),
            2usize => Ok(AtomName([s[0], s[1], b' ', b' '])),
            3usize => Ok(AtomName([s[0], s[1], s[2], b' '])),
            4usize => Ok(AtomName([s[0], s[1], s[2], s[3]])),
            0usize => Err(ParseAtomNameError::LengthZero),
            _ => Err(ParseAtomNameError::LengthGreaterThan4),
        }
    }
}

// #[derive(Debug, Clone, Serialize)]
// pub enum AtomName {
//     AminoAcid(AminoAcidAtomName),
//     Nucleotide(NucleotideAtomName),
//     WaterO,
//     Other(String),
// }

// #[derive(Debug, Clone, Serialize)]
// pub enum AminoAcidAtomName {
//     N,
//     CA,
//     C,
//     O,
//     Other(String),
// }

// impl AminoAcidAtomName {
//     pub fn from_bytes(inp: &[u8]) -> Self {
//         match inp {
//             b"N" => Self::N,
//             b"CA" => Self::CA,
//             b"C" => Self::C,
//             b"O" => Self::O,
//             _ => Self::Other(unsafe { std::str::from_utf8_unchecked(inp).to_owned() }),
//         }
//     }
//     pub fn from_bytes_fixed4(inp: &[u8]) -> Self {
//         match inp {
//             b" N  " => Self::N,
//             b" CA " => Self::CA,
//             b" C  " => Self::C,
//             b" O  " => Self::O,
//             _ => Self::Other(unsafe { std::str::from_utf8_unchecked(inp).trim_start().to_owned() }),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize)]
// pub enum NucleotideAtomName {
//     OP1,
//     OP2,
//     O5,
//     O4,
//     O3,
//     O2,
//     C5,
//     C4,
//     C3,
//     C2,
//     C1,
//     N9,
//     N7,
//     N6,
//     N4,
//     N3,
//     N2,
//     N1,
//     P,
//     Other(String),
// }

// impl NucleotideAtomName {
//     pub fn from_bytes_uppercase(inp: &[u8]) -> Self {
//         match inp {
//             b"OP1" => Self::OP1,
//             b"OP2" => Self::OP2,
//             b"O5" => Self::O5,
//             b"O4" => Self::O4,
//             b"O3" => Self::O3,
//             b"O2" => Self::O2,
//             b"C5" => Self::C5,
//             b"C4" => Self::C4,
//             b"C3" => Self::C3,
//             b"C2" => Self::C2,
//             b"C1" => Self::C1,
//             b"N9" => Self::N9,
//             b"N7" => Self::N7,
//             b"N6" => Self::N6,
//             b"N4" => Self::N4,
//             b"N3" => Self::N3,
//             b"N2" => Self::N2,
//             b"N1" => Self::N1,
//             b"P" => Self::P,
//             _ => Self::Other(unsafe { std::str::from_utf8_unchecked(inp).to_owned() }),
//         }
//     }
//     pub fn from_bytes_uppercase_fixed4(inp: &[u8]) -> Self {
//         match inp {
//             b" OP1" => Self::OP1,
//             b" OP2" => Self::OP2,
//             b" O5 " => Self::O5,
//             b" O4 " => Self::O4,
//             b" O3 " => Self::O3,
//             b" O2 " => Self::O2,
//             b" C5 " => Self::C5,
//             b" C4 " => Self::C4,
//             b" C3 " => Self::C3,
//             b" C2 " => Self::C2,
//             b" C1 " => Self::C1,
//             b" N9 " => Self::N9,
//             b" N7 " => Self::N7,
//             b" N6 " => Self::N6,
//             b" N4 " => Self::N4,
//             b" N3 " => Self::N3,
//             b" N2 " => Self::N2,
//             b" N1 " => Self::N1,
//             b" P  " => Self::P,
//             _ => Self::Other(unsafe { std::str::from_utf8_unchecked(inp).trim_start().to_owned() }),
//         }
//     }
// }
