/// Connectivity within a Ala residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ALA_CONNECT_NORMAL: [(&'static str, &'static str, u8); 9] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("1HB", "CB", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cym residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYM_CONNECT_NORMAL: [(&'static str, &'static str, u8); 9] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("3HB", "CB", 1),
    ("2HB", "CB", 1),
    ("SG", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYS_CONNECT_NORMAL: [(&'static str, &'static str, u8); 10] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("SG", "CB", 1),
    ("HG", "SG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cyx residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYX_CONNECT_NORMAL: [(&'static str, &'static str, u8); 9] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("SG", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Asp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASP_CONNECT_NORMAL: [(&'static str, &'static str, u8); 11] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("OD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ash residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASH_CONNECT_NORMAL: [(&'static str, &'static str, u8); 12] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("OD2", "CG", 1),
    ("OD2", "HD2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Glu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLU_CONNECT_NORMAL: [(&'static str, &'static str, u8); 14] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("OE2", "CD", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Glp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLP_CONNECT_NORMAL: [(&'static str, &'static str, u8); 15] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("OE2", "CD", 1),
    ("OE2", "HE2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Phe residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PHE_CONNECT_NORMAL: [(&'static str, &'static str, u8); 20] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("CE1", "CD1", 1),
    ("1HE", "CE1", 1),
    ("CZ", "CE1", 2),
    ("HZ", "CZ", 1),
    ("CE2", "CZ", 1),
    ("2HE", "CE2", 1),
    ("CD2", "CE2", 2),
    ("2HD", "CD2", 1),
    ("CD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Gly residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLY_CONNECT_NORMAL: [(&'static str, &'static str, u8); 6] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("CA", "3HA", 1),
];

/// Connectivity within a Hip residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIP_CONNECT_NORMAL: [(&'static str, &'static str, u8); 18] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("1HD", "ND1", 1),
    ("CE1", "ND1", 2),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 1),
    ("2HE", "NE2", 1),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Hie residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIE_CONNECT_NORMAL: [(&'static str, &'static str, u8); 17] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("CE1", "ND1", 2),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 1),
    ("2HE", "NE2", 1),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Hid residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HID_CONNECT_NORMAL: [(&'static str, &'static str, u8); 17] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("1HD", "ND1", 1),
    ("CE1", "ND1", 1),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 2),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Ile residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ILE_CONNECT_NORMAL: [(&'static str, &'static str, u8); 18] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("HB", "CB", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("CG1", "CB", 1),
    ("2HG1", "CG1", 1),
    ("3HG1", "CG1", 1),
    ("CD1", "CG1", 1),
    ("1HD1", "CD1", 1),
    ("2HD1", "CD1", 1),
    ("3HD1", "CD1", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Lys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LYS_CONNECT_NORMAL: [(&'static str, &'static str, u8); 21] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("2HD", "CD", 1),
    ("3HD", "CD", 1),
    ("CE", "CD", 1),
    ("2HE", "CE", 1),
    ("3HE", "CE", 1),
    ("NZ", "CE", 1),
    ("1HZ", "NZ", 1),
    ("2HZ", "NZ", 1),
    ("3HZ", "NZ", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Leu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LEU_CONNECT_NORMAL: [(&'static str, &'static str, u8); 18] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("HG", "CG", 1),
    ("CD1", "CG", 1),
    ("1HD1", "CD1", 1),
    ("2HD1", "CD1", 1),
    ("3HD1", "CD1", 1),
    ("CD2", "CG", 1),
    ("1HD2", "CD2", 1),
    ("2HD2", "CD2", 1),
    ("3HD2", "CD2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Met residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const MET_CONNECT_NORMAL: [(&'static str, &'static str, u8); 16] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("SD", "CG", 1),
    ("CE", "SD", 1),
    ("1HE", "CE", 1),
    ("2HE", "CE", 1),
    ("3HE", "CE", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Asn residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASN_CONNECT_NORMAL: [(&'static str, &'static str, u8); 13] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("ND2", "CG", 1),
    ("1HD2", "ND2", 1),
    ("2HD2", "ND2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Pro residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PRO_CONNECT_NORMAL: [(&'static str, &'static str, u8); 14] = [
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("3HD", "CD", 1),
    ("2HD", "CD", 1),
    ("CG", "CD", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CB", "CG", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CA", "CB", 1),
    ("N", "CD", 1),
];

/// Connectivity within a Gln residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLN_CONNECT_NORMAL: [(&'static str, &'static str, u8); 16] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("NE2", "CD", 1),
    ("1HE2", "NE2", 1),
    ("2HE2", "NE2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Arg residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ARG_CONNECT_NORMAL: [(&'static str, &'static str, u8); 23] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("2HD", "CD", 1),
    ("3HD", "CD", 1),
    ("NE", "CD", 1),
    ("HE", "NE", 1),
    ("CZ", "NE", 1),
    ("NH1", "CZ", 2),
    ("1HH1", "NH1", 1),
    ("2HH1", "NH1", 1),
    ("NH2", "CZ", 1),
    ("1HH2", "NH2", 1),
    ("2HH2", "NH2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ser residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const SER_CONNECT_NORMAL: [(&'static str, &'static str, u8); 10] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("OG", "CB", 1),
    ("HG", "OG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Thr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const THR_CONNECT_NORMAL: [(&'static str, &'static str, u8); 13] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("HB", "CB", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("OG1", "CB", 1),
    ("1HG", "OG1", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Val residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const VAL_CONNECT_NORMAL: [(&'static str, &'static str, u8); 15] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("HB", "CB", 1),
    ("CG1", "CB", 1),
    ("1HG1", "CG1", 1),
    ("2HG1", "CG1", 1),
    ("3HG1", "CG1", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Trp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TRP_CONNECT_NORMAL: [(&'static str, &'static str, u8); 25] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("NE1", "CD1", 1),
    ("1HE", "NE1", 1),
    ("CE2", "NE1", 1),
    ("CZ2", "CE2", 1),
    ("2HZ", "CZ2", 1),
    ("CH2", "CZ2", 2),
    ("2HH", "CH2", 1),
    ("CZ3", "CH2", 1),
    ("3HZ", "CZ3", 1),
    ("CE3", "CZ3", 2),
    ("3HE", "CE3", 1),
    ("CD2", "CE3", 1),
    ("CG", "CD2", 1),
    ("CE2", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Tyr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TYR_CONNECT_NORMAL: [(&'static str, &'static str, u8); 21] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("CE1", "CD1", 1),
    ("1HE", "CE1", 1),
    ("CZ", "CE1", 2),
    ("OH", "CZ", 1),
    ("HH", "OH", 1),
    ("CE2", "CZ", 1),
    ("2HE", "CE2", 1),
    ("CD2", "CE2", 2),
    ("2HD", "CD2", 1),
    ("CD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Nme residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const NME_CONNECT_NORMAL: [(&'static str, &'static str, u8); 5] = [
    ("N", "H", 1),
    ("N", "CH3", 1),
    ("CH3", "1HH3", 1),
    ("CH3", "2HH3", 1),
    ("CH3", "3HH3", 1),
];

/// Connectivity within a Ace residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ACE_CONNECT_NORMAL: [(&'static str, &'static str, u8); 5] = [
    ("CH3", "1HH3", 1),
    ("CH3", "2HH3", 1),
    ("CH3", "3HH3", 1),
    ("CH3", "C", 1),
    ("C", "O", 2),
];

/// Connectivity within a Ala residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ALA_CONNECT_NTERM: [(&'static str, &'static str, u8); 11] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("1HB", "CB", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYS_CONNECT_NTERM: [(&'static str, &'static str, u8); 12] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("SG", "CB", 1),
    ("HG", "SG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cyx residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYX_CONNECT_NTERM: [(&'static str, &'static str, u8); 11] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("SG", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Asp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASP_CONNECT_NTERM: [(&'static str, &'static str, u8); 13] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("OD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ash residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASH_CONNECT_NTERM: [(&'static str, &'static str, u8); 14] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("OD2", "CG", 1),
    ("OD2", "HD2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Glu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLU_CONNECT_NTERM: [(&'static str, &'static str, u8); 16] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("OE2", "CD", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Glp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLP_CONNECT_NTERM: [(&'static str, &'static str, u8); 17] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("OE2", "CD", 1),
    ("OE2", "HE2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Phe residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PHE_CONNECT_NTERM: [(&'static str, &'static str, u8); 22] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("CE1", "CD1", 1),
    ("1HE", "CE1", 1),
    ("CZ", "CE1", 2),
    ("HZ", "CZ", 1),
    ("CE2", "CZ", 1),
    ("2HE", "CE2", 1),
    ("CD2", "CE2", 2),
    ("2HD", "CD2", 1),
    ("CD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Gly residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLY_CONNECT_NTERM: [(&'static str, &'static str, u8); 8] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("CA", "3HA", 1),
];

/// Connectivity within a Hip residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIP_CONNECT_NTERM: [(&'static str, &'static str, u8); 20] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("1HD", "ND1", 1),
    ("CE1", "ND1", 2),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 1),
    ("2HE", "NE2", 1),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Hie residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIE_CONNECT_NTERM: [(&'static str, &'static str, u8); 19] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("CE1", "ND1", 2),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 1),
    ("2HE", "NE2", 1),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Hid residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HID_CONNECT_NTERM: [(&'static str, &'static str, u8); 19] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("1HD", "ND1", 1),
    ("CE1", "ND1", 1),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 2),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Ile residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ILE_CONNECT_NTERM: [(&'static str, &'static str, u8); 20] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("HB", "CB", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("CG1", "CB", 1),
    ("2HG1", "CG1", 1),
    ("3HG1", "CG1", 1),
    ("CD1", "CG1", 1),
    ("1HD1", "CD1", 1),
    ("2HD1", "CD1", 1),
    ("3HD1", "CD1", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Lys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LYS_CONNECT_NTERM: [(&'static str, &'static str, u8); 23] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("2HD", "CD", 1),
    ("3HD", "CD", 1),
    ("CE", "CD", 1),
    ("2HE", "CE", 1),
    ("3HE", "CE", 1),
    ("NZ", "CE", 1),
    ("1HZ", "NZ", 1),
    ("2HZ", "NZ", 1),
    ("3HZ", "NZ", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Leu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LEU_CONNECT_NTERM: [(&'static str, &'static str, u8); 20] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("HG", "CG", 1),
    ("CD1", "CG", 1),
    ("1HD1", "CD1", 1),
    ("2HD1", "CD1", 1),
    ("3HD1", "CD1", 1),
    ("CD2", "CG", 1),
    ("1HD2", "CD2", 1),
    ("2HD2", "CD2", 1),
    ("3HD2", "CD2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Met residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const MET_CONNECT_NTERM: [(&'static str, &'static str, u8); 18] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("SD", "CG", 1),
    ("CE", "SD", 1),
    ("1HE", "CE", 1),
    ("2HE", "CE", 1),
    ("3HE", "CE", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Asn residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASN_CONNECT_NTERM: [(&'static str, &'static str, u8); 15] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("ND2", "CG", 1),
    ("1HD2", "ND2", 1),
    ("2HD2", "ND2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Pro residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PRO_CONNECT_NTERM: [(&'static str, &'static str, u8); 16] = [
    ("N", "2H", 1),
    ("N", "3H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("3HD", "CD", 1),
    ("2HD", "CD", 1),
    ("CG", "CD", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CB", "CG", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CA", "CB", 1),
    ("N", "CD", 1),
];

/// Connectivity within a Gln residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLN_CONNECT_NTERM: [(&'static str, &'static str, u8); 18] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("NE2", "CD", 1),
    ("1HE2", "NE2", 1),
    ("2HE2", "NE2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Arg residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ARG_CONNECT_NTERM: [(&'static str, &'static str, u8); 25] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("2HD", "CD", 1),
    ("3HD", "CD", 1),
    ("NE", "CD", 1),
    ("HE", "NE", 1),
    ("CZ", "NE", 1),
    ("NH1", "CZ", 2),
    ("1HH1", "NH1", 1),
    ("2HH1", "NH1", 1),
    ("NH2", "CZ", 1),
    ("1HH2", "NH2", 1),
    ("2HH2", "NH2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ser residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const SER_CONNECT_NTERM: [(&'static str, &'static str, u8); 12] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("OG", "CB", 1),
    ("HG", "OG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Thr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const THR_CONNECT_NTERM: [(&'static str, &'static str, u8); 15] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("HB", "CB", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("OG1", "CB", 1),
    ("1HG", "OG1", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Val residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const VAL_CONNECT_NTERM: [(&'static str, &'static str, u8); 17] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("HB", "CB", 1),
    ("CG1", "CB", 1),
    ("1HG1", "CG1", 1),
    ("2HG1", "CG1", 1),
    ("3HG1", "CG1", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Trp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TRP_CONNECT_NTERM: [(&'static str, &'static str, u8); 27] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("NE1", "CD1", 1),
    ("1HE", "NE1", 1),
    ("CE2", "NE1", 1),
    ("CZ2", "CE2", 1),
    ("2HZ", "CZ2", 1),
    ("CH2", "CZ2", 2),
    ("2HH", "CH2", 1),
    ("CZ3", "CH2", 1),
    ("3HZ", "CZ3", 1),
    ("CE3", "CZ3", 2),
    ("3HE", "CE3", 1),
    ("CD2", "CE3", 1),
    ("CG", "CD2", 1),
    ("CE2", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Tyr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TYR_CONNECT_NTERM: [(&'static str, &'static str, u8); 23] = [
    ("N", "1HT", 1),
    ("N", "2HT", 1),
    ("N", "3HT", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("CE1", "CD1", 1),
    ("1HE", "CE1", 1),
    ("CZ", "CE1", 2),
    ("OH", "CZ", 1),
    ("HH", "OH", 1),
    ("CE2", "CZ", 1),
    ("2HE", "CE2", 1),
    ("CD2", "CE2", 2),
    ("2HD", "CD2", 1),
    ("CD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ala residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ALA_CONNECT_CTERM: [(&'static str, &'static str, u8); 10] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("1HB", "CB", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYS_CONNECT_CTERM: [(&'static str, &'static str, u8); 11] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("SG", "CB", 1),
    ("HG", "SG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Cyx residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const CYX_CONNECT_CTERM: [(&'static str, &'static str, u8); 10] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("SG", "CB", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Asp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASP_CONNECT_CTERM: [(&'static str, &'static str, u8); 12] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("OD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ash residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASH_CONNECT_CTERM: [(&'static str, &'static str, u8); 13] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("OD2", "CG", 1),
    ("OD2", "HD2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Glu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLU_CONNECT_CTERM: [(&'static str, &'static str, u8); 15] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("OE2", "CD", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Glp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLP_CONNECT_CTERM: [(&'static str, &'static str, u8); 16] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("OE2", "CD", 1),
    ("OE2", "HE2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Phe residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PHE_CONNECT_CTERM: [(&'static str, &'static str, u8); 21] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("CE1", "CD1", 1),
    ("1HE", "CE1", 1),
    ("CZ", "CE1", 2),
    ("HZ", "CZ", 1),
    ("CE2", "CZ", 1),
    ("2HE", "CE2", 1),
    ("CD2", "CE2", 2),
    ("2HD", "CD2", 1),
    ("CD2", "CG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Gly residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLY_CONNECT_CTERM: [(&'static str, &'static str, u8); 7] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("CA", "3HA", 1),
];

/// Connectivity within a Hip residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIP_CONNECT_CTERM: [(&'static str, &'static str, u8); 19] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("1HD", "ND1", 1),
    ("CE1", "ND1", 2),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 1),
    ("2HE", "NE2", 1),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Hie residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HIE_CONNECT_CTERM: [(&'static str, &'static str, u8); 18] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("CE1", "ND1", 2),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 1),
    ("2HE", "NE2", 1),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Hid residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const HID_CONNECT_CTERM: [(&'static str, &'static str, u8); 18] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("ND1", "CG", 1),
    ("1HD", "ND1", 1),
    ("CE1", "ND1", 1),
    ("1HE", "CE1", 1),
    ("NE2", "CE1", 2),
    ("CD2", "NE2", 1),
    ("2HD", "CD2", 1),
    ("CG", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Ile residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ILE_CONNECT_CTERM: [(&'static str, &'static str, u8); 19] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("HB", "CB", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("CG1", "CB", 1),
    ("2HG1", "CG1", 1),
    ("3HG1", "CG1", 1),
    ("CD1", "CG1", 1),
    ("1HD1", "CD1", 1),
    ("2HD1", "CD1", 1),
    ("3HD1", "CD1", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Lys residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LYS_CONNECT_CTERM: [(&'static str, &'static str, u8); 22] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("2HD", "CD", 1),
    ("3HD", "CD", 1),
    ("CE", "CD", 1),
    ("2HE", "CE", 1),
    ("3HE", "CE", 1),
    ("NZ", "CE", 1),
    ("1HZ", "NZ", 1),
    ("2HZ", "NZ", 1),
    ("3HZ", "NZ", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Leu residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const LEU_CONNECT_CTERM: [(&'static str, &'static str, u8); 19] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("HG", "CG", 1),
    ("CD1", "CG", 1),
    ("1HD1", "CD1", 1),
    ("2HD1", "CD1", 1),
    ("3HD1", "CD1", 1),
    ("CD2", "CG", 1),
    ("1HD2", "CD2", 1),
    ("2HD2", "CD2", 1),
    ("3HD2", "CD2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Met residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const MET_CONNECT_CTERM: [(&'static str, &'static str, u8); 17] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("SD", "CG", 1),
    ("CE", "SD", 1),
    ("1HE", "CE", 1),
    ("2HE", "CE", 1),
    ("3HE", "CE", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Asn residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ASN_CONNECT_CTERM: [(&'static str, &'static str, u8); 14] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("OD1", "CG", 2),
    ("ND2", "CG", 1),
    ("1HD2", "ND2", 1),
    ("2HD2", "ND2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Pro residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const PRO_CONNECT_CTERM: [(&'static str, &'static str, u8); 15] = [
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("3HD", "CD", 1),
    ("2HD", "CD", 1),
    ("CG", "CD", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CB", "CG", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CA", "CB", 1),
    ("N", "CD", 1),
];

/// Connectivity within a Gln residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const GLN_CONNECT_CTERM: [(&'static str, &'static str, u8); 17] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("OE1", "CD", 2),
    ("NE2", "CD", 1),
    ("1HE2", "NE2", 1),
    ("2HE2", "NE2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Arg residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const ARG_CONNECT_CTERM: [(&'static str, &'static str, u8); 24] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("2HG", "CG", 1),
    ("3HG", "CG", 1),
    ("CD", "CG", 1),
    ("2HD", "CD", 1),
    ("3HD", "CD", 1),
    ("NE", "CD", 1),
    ("HE", "NE", 1),
    ("CZ", "NE", 1),
    ("NH1", "CZ", 2),
    ("1HH1", "NH1", 1),
    ("2HH1", "NH1", 1),
    ("NH2", "CZ", 1),
    ("1HH2", "NH2", 1),
    ("2HH2", "NH2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Ser residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const SER_CONNECT_CTERM: [(&'static str, &'static str, u8); 11] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("OG", "CB", 1),
    ("HG", "OG", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Thr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const THR_CONNECT_CTERM: [(&'static str, &'static str, u8); 14] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("HB", "CB", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("OG1", "CB", 1),
    ("1HG", "OG1", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Val residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const VAL_CONNECT_CTERM: [(&'static str, &'static str, u8); 16] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("HB", "CB", 1),
    ("CG1", "CB", 1),
    ("1HG1", "CG1", 1),
    ("2HG1", "CG1", 1),
    ("3HG1", "CG1", 1),
    ("CG2", "CB", 1),
    ("1HG2", "CG2", 1),
    ("2HG2", "CG2", 1),
    ("3HG2", "CG2", 1),
    ("CA", "CB", 1),
];

/// Connectivity within a Trp residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TRP_CONNECT_CTERM: [(&'static str, &'static str, u8); 26] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("NE1", "CD1", 1),
    ("1HE", "NE1", 1),
    ("CE2", "NE1", 1),
    ("CZ2", "CE2", 1),
    ("2HZ", "CZ2", 1),
    ("CH2", "CZ2", 2),
    ("2HH", "CH2", 1),
    ("CZ3", "CH2", 1),
    ("3HZ", "CZ3", 1),
    ("CE3", "CZ3", 2),
    ("3HE", "CE3", 1),
    ("CD2", "CE3", 1),
    ("CG", "CD2", 1),
    ("CE2", "CD2", 2),
    ("CA", "CB", 1),
];

/// Connectivity within a Tyr residue.
/// Each tuple represents `(AtomNameX, AtomNameY, BondOrder)`.
pub const TYR_CONNECT_CTERM: [(&'static str, &'static str, u8); 22] = [
    ("N", "H", 1),
    ("N", "CA", 1),
    ("CA", "HA", 1),
    ("CA", "C", 1),
    ("C", "O", 2),
    ("C", "OXT", 1),
    ("2HB", "CB", 1),
    ("3HB", "CB", 1),
    ("CG", "CB", 1),
    ("CD1", "CG", 2),
    ("1HD", "CD1", 1),
    ("CE1", "CD1", 1),
    ("1HE", "CE1", 1),
    ("CZ", "CE1", 2),
    ("OH", "CZ", 1),
    ("HH", "OH", 1),
    ("CE2", "CZ", 1),
    ("2HE", "CE2", 1),
    ("CD2", "CE2", 2),
    ("2HD", "CD2", 1),
    ("CD2", "CG", 1),
    ("CA", "CB", 1),
];
