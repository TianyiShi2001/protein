use crate::structure::AtomName;

/// Connectivity within a Ala residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ALA_CONNECT_NORMAL: [(AtomName, AtomName, u8); 9] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"1HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cym residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYM_CONNECT_NORMAL: [(AtomName, AtomName, u8); 9] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYS_CONNECT_NORMAL: [(AtomName, AtomName, u8); 10] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"SG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cyx residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYX_CONNECT_NORMAL: [(AtomName, AtomName, u8); 9] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Asp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASP_CONNECT_NORMAL: [(AtomName, AtomName, u8); 11] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"OD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ash residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASH_CONNECT_NORMAL: [(AtomName, AtomName, u8); 12] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"OD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OD2 "), AtomName(*b"HD2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Glu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLU_CONNECT_NORMAL: [(AtomName, AtomName, u8); 14] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"OE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Glp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLP_CONNECT_NORMAL: [(AtomName, AtomName, u8); 15] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"OE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"OE2 "), AtomName(*b"HE2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Phe residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PHE_CONNECT_NORMAL: [(AtomName, AtomName, u8); 20] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"HZ  "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE2 "), 2),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Gly residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLY_CONNECT_NORMAL: [(AtomName, AtomName, u8); 6] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"CA  "), AtomName(*b"3HA "), 1),
];

/// Connectivity within a Hip residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIP_CONNECT_NORMAL: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 2),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"2HE "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Hie residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIE_CONNECT_NORMAL: [(AtomName, AtomName, u8); 17] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 2),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"2HE "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Hid residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HID_CONNECT_NORMAL: [(AtomName, AtomName, u8); 17] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ile residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ILE_CONNECT_NORMAL: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"CG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"3HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG1 "), 1),
    (AtomName(*b"1HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"2HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"3HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Lys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LYS_CONNECT_NORMAL: [(AtomName, AtomName, u8); 21] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CE  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"3HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"NZ  "), AtomName(*b"CE  "), 1),
    (AtomName(*b"1HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"2HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"3HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Leu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LEU_CONNECT_NORMAL: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"2HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"3HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"2HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"3HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Met residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const MET_CONNECT_NORMAL: [(AtomName, AtomName, u8); 16] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"SD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CE  "), AtomName(*b"SD  "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"3HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Asn residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASN_CONNECT_NORMAL: [(AtomName, AtomName, u8); 13] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"ND2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD2"), AtomName(*b"ND2 "), 1),
    (AtomName(*b"2HD2"), AtomName(*b"ND2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Pro residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PRO_CONNECT_NORMAL: [(AtomName, AtomName, u8); 14] = [
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CB  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"N   "), AtomName(*b"CD  "), 1),
];

/// Connectivity within a Gln residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLN_CONNECT_NORMAL: [(AtomName, AtomName, u8); 16] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"NE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"1HE2"), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HE2"), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Arg residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ARG_CONNECT_NORMAL: [(AtomName, AtomName, u8); 23] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"NE  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"HE  "), AtomName(*b"NE  "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"NE  "), 1),
    (AtomName(*b"NH1 "), AtomName(*b"CZ  "), 2),
    (AtomName(*b"1HH1"), AtomName(*b"NH1 "), 1),
    (AtomName(*b"2HH1"), AtomName(*b"NH1 "), 1),
    (AtomName(*b"NH2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"1HH2"), AtomName(*b"NH2 "), 1),
    (AtomName(*b"2HH2"), AtomName(*b"NH2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ser residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const SER_CONNECT_NORMAL: [(AtomName, AtomName, u8); 10] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"OG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Thr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const THR_CONNECT_NORMAL: [(AtomName, AtomName, u8); 13] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"OG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG "), AtomName(*b"OG1 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Val residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const VAL_CONNECT_NORMAL: [(AtomName, AtomName, u8); 15] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"2HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"3HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Trp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TRP_CONNECT_NORMAL: [(AtomName, AtomName, u8); 25] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"NE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"NE1 "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"NE1 "), 1),
    (AtomName(*b"CZ2 "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"2HZ "), AtomName(*b"CZ2 "), 1),
    (AtomName(*b"CH2 "), AtomName(*b"CZ2 "), 2),
    (AtomName(*b"2HH "), AtomName(*b"CH2 "), 1),
    (AtomName(*b"CZ3 "), AtomName(*b"CH2 "), 1),
    (AtomName(*b"3HZ "), AtomName(*b"CZ3 "), 1),
    (AtomName(*b"CE3 "), AtomName(*b"CZ3 "), 2),
    (AtomName(*b"3HE "), AtomName(*b"CE3 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE3 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Tyr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TYR_CONNECT_NORMAL: [(AtomName, AtomName, u8); 21] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"OH  "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"HH  "), AtomName(*b"OH  "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE2 "), 2),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Nme residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const NME_CONNECT_NORMAL: [(AtomName, AtomName, u8); 5] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CH3 "), 1),
    (AtomName(*b"CH3 "), AtomName(*b"1HH3"), 1),
    (AtomName(*b"CH3 "), AtomName(*b"2HH3"), 1),
    (AtomName(*b"CH3 "), AtomName(*b"3HH3"), 1),
];

/// Connectivity within a Ace residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ACE_CONNECT_NORMAL: [(AtomName, AtomName, u8); 5] = [
    (AtomName(*b"CH3 "), AtomName(*b"1HH3"), 1),
    (AtomName(*b"CH3 "), AtomName(*b"2HH3"), 1),
    (AtomName(*b"CH3 "), AtomName(*b"3HH3"), 1),
    (AtomName(*b"CH3 "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
];

/// Connectivity within a Ala residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ALA_CONNECT_NTERM: [(AtomName, AtomName, u8); 11] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"1HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYS_CONNECT_NTERM: [(AtomName, AtomName, u8); 12] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"SG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cyx residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYX_CONNECT_NTERM: [(AtomName, AtomName, u8); 11] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Asp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASP_CONNECT_NTERM: [(AtomName, AtomName, u8); 13] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"OD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ash residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASH_CONNECT_NTERM: [(AtomName, AtomName, u8); 14] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"OD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OD2 "), AtomName(*b"HD2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Glu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLU_CONNECT_NTERM: [(AtomName, AtomName, u8); 16] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"OE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Glp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLP_CONNECT_NTERM: [(AtomName, AtomName, u8); 17] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"OE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"OE2 "), AtomName(*b"HE2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Phe residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PHE_CONNECT_NTERM: [(AtomName, AtomName, u8); 22] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"HZ  "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE2 "), 2),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Gly residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLY_CONNECT_NTERM: [(AtomName, AtomName, u8); 8] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"CA  "), AtomName(*b"3HA "), 1),
];

/// Connectivity within a Hip residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIP_CONNECT_NTERM: [(AtomName, AtomName, u8); 20] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 2),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"2HE "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Hie residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIE_CONNECT_NTERM: [(AtomName, AtomName, u8); 19] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 2),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"2HE "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Hid residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HID_CONNECT_NTERM: [(AtomName, AtomName, u8); 19] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ile residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ILE_CONNECT_NTERM: [(AtomName, AtomName, u8); 20] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"CG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"3HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG1 "), 1),
    (AtomName(*b"1HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"2HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"3HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Lys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LYS_CONNECT_NTERM: [(AtomName, AtomName, u8); 23] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CE  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"3HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"NZ  "), AtomName(*b"CE  "), 1),
    (AtomName(*b"1HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"2HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"3HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Leu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LEU_CONNECT_NTERM: [(AtomName, AtomName, u8); 20] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"2HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"3HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"2HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"3HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Met residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const MET_CONNECT_NTERM: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"SD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CE  "), AtomName(*b"SD  "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"3HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Asn residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASN_CONNECT_NTERM: [(AtomName, AtomName, u8); 15] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"ND2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD2"), AtomName(*b"ND2 "), 1),
    (AtomName(*b"2HD2"), AtomName(*b"ND2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Pro residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PRO_CONNECT_NTERM: [(AtomName, AtomName, u8); 16] = [
    (AtomName(*b"N   "), AtomName(*b"2H  "), 1),
    (AtomName(*b"N   "), AtomName(*b"3H  "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CB  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"N   "), AtomName(*b"CD  "), 1),
];

/// Connectivity within a Gln residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLN_CONNECT_NTERM: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"NE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"1HE2"), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HE2"), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Arg residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ARG_CONNECT_NTERM: [(AtomName, AtomName, u8); 25] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"NE  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"HE  "), AtomName(*b"NE  "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"NE  "), 1),
    (AtomName(*b"NH1 "), AtomName(*b"CZ  "), 2),
    (AtomName(*b"1HH1"), AtomName(*b"NH1 "), 1),
    (AtomName(*b"2HH1"), AtomName(*b"NH1 "), 1),
    (AtomName(*b"NH2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"1HH2"), AtomName(*b"NH2 "), 1),
    (AtomName(*b"2HH2"), AtomName(*b"NH2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ser residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const SER_CONNECT_NTERM: [(AtomName, AtomName, u8); 12] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"OG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Thr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const THR_CONNECT_NTERM: [(AtomName, AtomName, u8); 15] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"OG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG "), AtomName(*b"OG1 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Val residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const VAL_CONNECT_NTERM: [(AtomName, AtomName, u8); 17] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"2HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"3HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Trp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TRP_CONNECT_NTERM: [(AtomName, AtomName, u8); 27] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"NE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"NE1 "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"NE1 "), 1),
    (AtomName(*b"CZ2 "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"2HZ "), AtomName(*b"CZ2 "), 1),
    (AtomName(*b"CH2 "), AtomName(*b"CZ2 "), 2),
    (AtomName(*b"2HH "), AtomName(*b"CH2 "), 1),
    (AtomName(*b"CZ3 "), AtomName(*b"CH2 "), 1),
    (AtomName(*b"3HZ "), AtomName(*b"CZ3 "), 1),
    (AtomName(*b"CE3 "), AtomName(*b"CZ3 "), 2),
    (AtomName(*b"3HE "), AtomName(*b"CE3 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE3 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Tyr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TYR_CONNECT_NTERM: [(AtomName, AtomName, u8); 23] = [
    (AtomName(*b"N   "), AtomName(*b"1HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"2HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"3HT "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"OH  "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"HH  "), AtomName(*b"OH  "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE2 "), 2),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ala residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ALA_CONNECT_CTERM: [(AtomName, AtomName, u8); 10] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"1HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYS_CONNECT_CTERM: [(AtomName, AtomName, u8); 11] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"SG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Cyx residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYX_CONNECT_CTERM: [(AtomName, AtomName, u8); 10] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"SG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Asp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASP_CONNECT_CTERM: [(AtomName, AtomName, u8); 12] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"OD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ash residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASH_CONNECT_CTERM: [(AtomName, AtomName, u8); 13] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"OD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OD2 "), AtomName(*b"HD2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Glu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLU_CONNECT_CTERM: [(AtomName, AtomName, u8); 15] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"OE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Glp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLP_CONNECT_CTERM: [(AtomName, AtomName, u8); 16] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"OE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"OE2 "), AtomName(*b"HE2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Phe residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PHE_CONNECT_CTERM: [(AtomName, AtomName, u8); 21] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"HZ  "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE2 "), 2),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Gly residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLY_CONNECT_CTERM: [(AtomName, AtomName, u8); 7] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"CA  "), AtomName(*b"3HA "), 1),
];

/// Connectivity within a Hip residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIP_CONNECT_CTERM: [(AtomName, AtomName, u8); 19] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 2),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"2HE "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Hie residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIE_CONNECT_CTERM: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 2),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"2HE "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Hid residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HID_CONNECT_CTERM: [(AtomName, AtomName, u8); 18] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"ND1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"ND1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"NE2 "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"CD2 "), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ile residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ILE_CONNECT_CTERM: [(AtomName, AtomName, u8); 19] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"CG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"3HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG1 "), 1),
    (AtomName(*b"1HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"2HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"3HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Lys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LYS_CONNECT_CTERM: [(AtomName, AtomName, u8); 22] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CE  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"3HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"NZ  "), AtomName(*b"CE  "), 1),
    (AtomName(*b"1HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"2HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"3HZ "), AtomName(*b"NZ  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Leu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LEU_CONNECT_CTERM: [(AtomName, AtomName, u8); 19] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"2HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"3HD1"), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"2HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"3HD2"), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Met residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const MET_CONNECT_CTERM: [(AtomName, AtomName, u8); 17] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"SD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CE  "), AtomName(*b"SD  "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"3HE "), AtomName(*b"CE  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Asn residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASN_CONNECT_CTERM: [(AtomName, AtomName, u8); 14] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"ND2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"1HD2"), AtomName(*b"ND2 "), 1),
    (AtomName(*b"2HD2"), AtomName(*b"ND2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Pro residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PRO_CONNECT_CTERM: [(AtomName, AtomName, u8); 15] = [
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CB  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"N   "), AtomName(*b"CD  "), 1),
];

/// Connectivity within a Gln residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLN_CONNECT_CTERM: [(AtomName, AtomName, u8); 17] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"OE1 "), AtomName(*b"CD  "), 2),
    (AtomName(*b"NE2 "), AtomName(*b"CD  "), 1),
    (AtomName(*b"1HE2"), AtomName(*b"NE2 "), 1),
    (AtomName(*b"2HE2"), AtomName(*b"NE2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Arg residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ARG_CONNECT_CTERM: [(AtomName, AtomName, u8); 24] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"2HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"3HG "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CD  "), AtomName(*b"CG  "), 1),
    (AtomName(*b"2HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"3HD "), AtomName(*b"CD  "), 1),
    (AtomName(*b"NE  "), AtomName(*b"CD  "), 1),
    (AtomName(*b"HE  "), AtomName(*b"NE  "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"NE  "), 1),
    (AtomName(*b"NH1 "), AtomName(*b"CZ  "), 2),
    (AtomName(*b"1HH1"), AtomName(*b"NH1 "), 1),
    (AtomName(*b"2HH1"), AtomName(*b"NH1 "), 1),
    (AtomName(*b"NH2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"1HH2"), AtomName(*b"NH2 "), 1),
    (AtomName(*b"2HH2"), AtomName(*b"NH2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Ser residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const SER_CONNECT_CTERM: [(AtomName, AtomName, u8); 11] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"OG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"HG  "), AtomName(*b"OG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Thr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const THR_CONNECT_CTERM: [(AtomName, AtomName, u8); 14] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"OG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG "), AtomName(*b"OG1 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Val residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const VAL_CONNECT_CTERM: [(AtomName, AtomName, u8); 16] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"HB  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG1 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"2HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"3HG1"), AtomName(*b"CG1 "), 1),
    (AtomName(*b"CG2 "), AtomName(*b"CB  "), 1),
    (AtomName(*b"1HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"2HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"3HG2"), AtomName(*b"CG2 "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Trp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TRP_CONNECT_CTERM: [(AtomName, AtomName, u8); 26] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"NE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"NE1 "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"NE1 "), 1),
    (AtomName(*b"CZ2 "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"2HZ "), AtomName(*b"CZ2 "), 1),
    (AtomName(*b"CH2 "), AtomName(*b"CZ2 "), 2),
    (AtomName(*b"2HH "), AtomName(*b"CH2 "), 1),
    (AtomName(*b"CZ3 "), AtomName(*b"CH2 "), 1),
    (AtomName(*b"3HZ "), AtomName(*b"CZ3 "), 1),
    (AtomName(*b"CE3 "), AtomName(*b"CZ3 "), 2),
    (AtomName(*b"3HE "), AtomName(*b"CE3 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE3 "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CD2 "), 2),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];

/// Connectivity within a Tyr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TYR_CONNECT_CTERM: [(AtomName, AtomName, u8); 22] = [
    (AtomName(*b"N   "), AtomName(*b"H   "), 1),
    (AtomName(*b"N   "), AtomName(*b"CA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"HA  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"C   "), 1),
    (AtomName(*b"C   "), AtomName(*b"O   "), 2),
    (AtomName(*b"C   "), AtomName(*b"OXT "), 1),
    (AtomName(*b"2HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"3HB "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CG  "), AtomName(*b"CB  "), 1),
    (AtomName(*b"CD1 "), AtomName(*b"CG  "), 2),
    (AtomName(*b"1HD "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"CE1 "), AtomName(*b"CD1 "), 1),
    (AtomName(*b"1HE "), AtomName(*b"CE1 "), 1),
    (AtomName(*b"CZ  "), AtomName(*b"CE1 "), 2),
    (AtomName(*b"OH  "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"HH  "), AtomName(*b"OH  "), 1),
    (AtomName(*b"CE2 "), AtomName(*b"CZ  "), 1),
    (AtomName(*b"2HE "), AtomName(*b"CE2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CE2 "), 2),
    (AtomName(*b"2HD "), AtomName(*b"CD2 "), 1),
    (AtomName(*b"CD2 "), AtomName(*b"CG  "), 1),
    (AtomName(*b"CA  "), AtomName(*b"CB  "), 1),
];
