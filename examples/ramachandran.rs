use csv::Writer; // the crate `csv` is required if you want to output csv
use protein::{
    analysis::model::ModelAnalysis, // `Structure` alone only stores data.
    get::get_pdb,
    // Functions for analysing the `Structure` are provided by separate traits
    io::parse_pdb, // the PDB parser that parses PDB file into a `Structure`
};

fn main() {
    let pdbfile = get_pdb("4f7i").unwrap();
    let structure = parse_pdb(&pdbfile).unwrap();
    let (phis, psis) = structure.models[0].ramachandran();
    // the `.ramachandran()` function is provided by the `ModelAnalysis` trait
    // this produces vectors of phi and psi angles in radians

    // the code below is used to output csv, which is optional
    let mut wtr = Writer::from_path("examples/ramachandran.csv").unwrap();
    wtr.write_record(&["phi", "psi"]).unwrap();
    for (&phi, &psi) in phis.iter().zip(psis.iter()) {
        wtr.write_record(&[phi.to_string(), psi.to_string()])
            .unwrap()
    }
    wtr.flush().unwrap();
}
