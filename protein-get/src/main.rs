use flate2::read::GzDecoder;
use std::io::prelude::*;

fn main() {
    let mut d = GzDecoder::new(&include_bytes!("../4hhb.pdb.gz")[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
