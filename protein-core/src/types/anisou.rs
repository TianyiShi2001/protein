use super::AtomSerial;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Anisou {
    pub id: AtomSerial,
    pub u11: i32,
    pub u22: i32,
    pub u33: i32,
    pub u12: i32,
    pub u13: i32,
    pub u23: i32,
}
