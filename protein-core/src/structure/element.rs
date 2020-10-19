use serde::Serialize;

/// An element in the periodic table that actually occurs in protein structure models.
///
/// String conversion methods are *not* provided because they are varied.
#[derive(Debug, Clone, Copy, Serialize)]
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
    Unknown,
}
