use serde::Serialize;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Clone, Copy, Serialize, EnumString)]
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

impl Element {
    pub fn from_uppercase(inp: &str) -> Self {
        Self::from_bytes_uppercase(inp.as_bytes())
    }

    pub fn from_bytes_uppercase(inp: &[u8]) -> Self {
        match inp {
            b"H" => Self::H,
            b"C" => Self::C,
            b"O" => Self::O,
            b"N" => Self::N,
            b"P" => Self::P,
            b"S" => Self::S,
            b"NA" => Self::Na,
            b"MG" => Self::Mg,
            b"CL" => Self::Cl,
            b"K" => Self::K,
            b"CA" => Self::Ca,
            b"FE" => Self::Fe,
            b"MN" => Self::Mn,
            b"CO" => Self::Co,
            b"CR" => Self::Cr,
            b"I" => Self::I,
            b"ZN" => Self::Zn,
            b"CU" => Self::Cu,
            b"F" => Self::F,
            b"AL" => Self::Al,
            b"SE" => Self::Se,
            b"V" => Self::V,
            _ => panic!(format!(
                "fail to parse element: {}",
                std::str::from_utf8(inp).unwrap()
            )),
        }
    }
    pub fn from_uppercase_fixed2(inp: &str) -> Self {
        Self::from_bytes_uppercase_fixed2(inp.as_bytes())
    }
    pub fn from_bytes_uppercase_fixed2(inp: &[u8]) -> Self {
        match inp {
            b" H" => Self::H,
            b" C" => Self::C,
            b" O" => Self::O,
            b" N" => Self::N,
            b" P" => Self::P,
            b" S" => Self::S,
            b"NA" => Self::Na,
            b"MG" => Self::Mg,
            b"CL" => Self::Cl,
            b" K" => Self::K,
            b"CA" => Self::Ca,
            b"FE" => Self::Fe,
            b"MN" => Self::Mn,
            b"CO" => Self::Co,
            b"CR" => Self::Cr,
            b" I" => Self::I,
            b"ZN" => Self::Zn,
            b"CU" => Self::Cu,
            b" F" => Self::F,
            b"AL" => Self::Al,
            b"SE" => Self::Se,
            b" V" => Self::V,
            _ => panic!(format!(
                "fail to parse element: {}",
                std::str::from_utf8(inp).unwrap()
            )),
        }
    }
}
