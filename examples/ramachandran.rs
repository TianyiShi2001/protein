use csv::Writer; // the crate `csv` is required if you want to output csv
use protein::{
    analysis::ModelAnalysis, // `Structure` alone only stores data.
    // Functions for analysing the `Structure` are provided by separate traits
    io::pdb::Parser, // the PDB parser that parses PDB file into a `Structure`
};
use protein_get::get_pdb;

fn main() {
    let pdbfile = get_pdb("4f7i").unwrap();
    let structure = Parser::parse(&pdbfile).unwrap();
    let (phis, psis) = structure.models[0].ramachandran(); // the `.ramachandran()` function is provided by the `ModelAnalysis` trait

    let mut wtr = Writer::from_path("examples/ramachandran.csv").unwrap();
    wtr.write_record(&["phi", "psi"]).unwrap();
    for (&phi, &psi) in phis.iter().zip(psis.iter()) {
        wtr.write_record(&[phi.to_string(), psi.to_string()])
            .unwrap()
    }
    wtr.flush().unwrap();
}
