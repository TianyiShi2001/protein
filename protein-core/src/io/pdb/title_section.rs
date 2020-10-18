use chrono::NaiveDate;
use serde::Serialize;

pub type Title = String;
pub type Keywords = Vec<String>;

/// Parses AUTHOR record which is a multiline continuation record.
/// Contains comma-seperated list of author names. If successfull
/// returns [Record](../ast/types/enum.Record.html) variant containing
/// [AUTHORS](../ast/types/struct.Authors.html) instance.
/// # Record structure
/// | COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                   |
/// |---------|--------------|--------------|----------------------------------------------|
/// | 1 -  6  | Record name  | AUTHOR       |                                              |
/// | 9 - 10  | Continuation | continuation | Allows concatenation of multiple records.    |
/// | 11 - 79 | List         | authorList   | List of the author names, separated          |
/// |         |              |              | by commas.                                   |
pub type Authors = Vec<String>;

/// The HEADER record uniquely identifies a PDB entry through the idCode field.
/// This record also provides a classification for the entry. Finally, it contains
/// the date when the coordinates were deposited to the PDB archive.
///
/// # Record Format
///
/// | COLUMNS | DATA  TYPE   | FIELD          | DEFINITION                                |
/// |---------|--------------|----------------|-------------------------------------------|
/// | 1 -  6  | Record name  | HEADER         |                                           |
/// | 11 - 50 | String(40)/`String`   | `classification` | Classifies the molecule(s).               |
/// | 51 - 59 | Date/`chrono::NaiveDate`         | `deposition_date`        | Deposition date. This is the date the coordinates  were received at the PDB.   |
/// | 63 - 66 | IDcode/`String`      | `id_code`         | This identifier is unique within the PDB. |
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Header {
    pub classification: String,
    pub deposition_date: NaiveDate,
    pub id_code: String,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            classification: String::new(),
            deposition_date: NaiveDate::from_ymd(1900, 1, 1),
            id_code: String::new(),
        }
    }
}

/// # Record structure
///
/// | COLUMNS | DATA TYPE     | FIELD        | DEFINITION                                |
/// |---------|---------------|--------------|-------------------------------------------|
/// | 1 -  6  | Record name   | EXPDTA       |                                           |
/// | 9 - 10  | Continuation  | continuation | Allows concatenation of multiple records. |
/// | 11 - 79 | SList         | technique    | The experimental technique(s) with        |
/// |         |                              | optional comment desc                     |
pub type ExperimentalTechniques = Vec<ExperimentalTechnique>;

#[derive(Debug, Clone, Serialize)]
pub enum ExperimentalTechnique {
    XRayDiffraction,
    ElectronMicroscopy,
    SolidStateNmr,
    SolutionNmr,
    NeutronDiffraction,
    ElectronCrystallography,
    SolutionScattering,
    FiberDiffraction,
}

impl std::str::FromStr for ExperimentalTechnique {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "X-RAY DIFFRACTION" => Ok(ExperimentalTechnique::XRayDiffraction),
            "ELECTRON MICROSCOPY" => Ok(ExperimentalTechnique::ElectronMicroscopy),
            "SOLID-STATE NMR" => Ok(ExperimentalTechnique::SolidStateNmr),
            "SOLUTION NMR" => Ok(ExperimentalTechnique::SolutionNmr),
            "NEUTRON DIFFRACTION" => Ok(ExperimentalTechnique::NeutronDiffraction),
            "ELECTRON CRYSTALLOGRAPHY" => Ok(ExperimentalTechnique::ElectronCrystallography),
            "SOLUTION SCATTERING" => Ok(ExperimentalTechnique::SolutionScattering),
            "FIBER DIFFRACTION" => Ok(ExperimentalTechnique::FiberDiffraction),
            _ => Err(format!("Unknown experimental result {}", inp)),
        }
    }
}
