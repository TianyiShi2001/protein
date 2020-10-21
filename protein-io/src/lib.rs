use protein_core::structure::Structure;
use std::error::Error;

pub fn parse_pdb(input: &[u8]) -> Result<Structure, Box<dyn Error + '_>> {
    nom_pdb::Parser::parse(input)
}
