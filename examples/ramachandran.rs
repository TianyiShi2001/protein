use csv::Writer;
use nom_pdb::complete::Parser;
use std::error::Error;

fn main() {
    let data = include_bytes!("../assets/4f7i.pdb");
    let (_, structure) = Parser::parse(data).unwrap();
    // let pretty = serde_json::to_string_pretty(&res).unwrap();
    let (phis, psis) = structure.models[0].ramachandran();

    let mut wtr = Writer::from_path("examples/ramachandran.csv").unwrap();
    wtr.write_record(&["phi", "psi"]).unwrap();
    for (&phi, &psi) in phis.iter().zip(psis.iter()) {
        wtr.write_record(&[phi.to_string(), psi.to_string()])
            .unwrap()
    }
    wtr.flush().unwrap();
}
