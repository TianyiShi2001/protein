use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::io::prelude::*;

/// Get the PDB file of the given entry code.
pub fn get_pdb(code: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let zipped = get(&format!("https://files.rcsb.org/download/{}.pdb.gz", code))?.bytes()?;
    let mut unzipped = Vec::<u8>::new();
    GzDecoder::new(&zipped[..]).read_to_end(&mut unzipped)?;
    Ok(unzipped)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_success() {
        let pdb1a8o = get_pdb("1A8O").unwrap();
        assert_eq!(&pdb1a8o[62..66], b"1A8O");
    }
}
