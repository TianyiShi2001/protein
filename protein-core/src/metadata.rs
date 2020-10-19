use serde::Serialize;
pub mod title_section;
pub use title_section::*;
pub mod crystallography;
pub use crystallography::*;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Metadata {
    pub header: Option<Header>,
    pub title: Option<Title>,
    pub authors: Option<Authors>,
    pub experimental_techniques: Option<ExperimentalTechniques>,
    pub cryst1: Option<Cryst1>,
}
