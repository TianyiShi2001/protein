use serde::{Deserialize, Serialize};

/// [Cryst1](www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1)
/// The CRYST1 record presents the unit cell parameters, space group, and Z value. If the
/// structure was not determined by crystallographic means, CRYST1 simply provides the unitary
/// values, with an appropriate REMARK.
///
/// # Record Format
///
/// COLUMNS DATA TYPE FIELD DEFINITION
/// ------------------------------------------------------------
/// 1  - 6  Record    name   "CRYST1"
/// 7  - 15 Real(9.3) a      a (Angstroms).
/// 16 - 24 Real(9.3) b      b (Angstroms).
/// 25 - 33 Real(9.3) c      c (Angstroms).
/// 34 - 40 Real(7.2) alpha  alpha (degrees).
/// 41 - 47 Real(7.2) beta   beta (degrees).
/// 48 - 54 Real(7.2) gamma  gamma (degrees).
/// 56 - 66 LString   sGroup Space group.
/// 67 - 70 Integer   z      Z value.
///
/// # Additional References
///
/// - https://infogalactic.com/info/Hermann%E2%80%93Mauguin_notation
/// - https://enacademic.com/dic.nsf/enwiki/1879109
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Cryst1 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub lattice_type: LatticeType,
    pub space_group: SpaceGroup,
    pub z: u8,
}

// Space groups can be defined by combining the point group identifier with the uppercase P, C, I,
// or F for primitive, side-centered, body-centered, or face-centered lattices.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LatticeType {
    Primitive,
    SideCentered,
    BodyCentered,
    FaceCentered,
    Unknown,
}

impl Default for LatticeType {
    fn default() -> Self {
        LatticeType::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GroupAxis(pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SpaceGroup(pub GroupAxis, pub Option<GroupAxis>, pub Option<GroupAxis>);
