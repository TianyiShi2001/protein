# protein

[![crates.io](https://img.shields.io/crates/d/protein.svg)](https://crates.io/crates/protein)
[![crates.io](https://img.shields.io/crates/v/protein.svg)](https://crates.io/crates/protein)
[![crates.io](https://img.shields.io/crates/l/protein.svg)](https://crates.io/crates/protein)
[![docs.rs](https://docs.rs/protein/badge.svg)](https://docs.rs/protein)

Protein structural Biology in Rust.

**NOTE: This crate is in early development and the API has not yet been stabilized, so do not use this crate in production. If you have any suggestions, please don't hesitate to open an issue or make a PR!**

## Components


| **crate**            | Links                                                                                                                                                                                                                                                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **protein**          | [![crates.io](https://img.shields.io/crates/d/protein.svg)](https://crates.io/crates/protein)[![crates.io](https://img.shields.io/crates/v/protein.svg)](https://crates.io/crates/protein)[![docs.rs](https://docs.rs/protein/badge.svg)](https://docs.rs/protein)                                                       |
| **protein-core**     | [![crates.io](https://img.shields.io/crates/d/protein-core.svg)](https://crates.io/crates/protein-core)[![crates.io](https://img.shields.io/crates/v/protein-core.svg)](https://crates.io/crates/protein-core)[![docs.rs](https://docs.rs/protein-core/badge.svg)](https://docs.rs/protein-core)                         |
| **protein-get**      | [![crates.io](https://img.shields.io/crates/d/protein-get.svg)](https://crates.io/crates/protein-get)[![crates.io](https://img.shields.io/crates/v/protein-get.svg)](https://crates.io/crates/protein-get)[![docs.rs](https://docs.rs/protein-get/badge.svg)](https://docs.rs/protein-get)                               |
| **protein-io**       | [![crates.io](https://img.shields.io/crates/d/protein-io.svg)](https://crates.io/crates/protein-io)[![crates.io](https://img.shields.io/crates/v/protein-io.svg)](https://crates.io/crates/protein-io)[![docs.rs](https://docs.rs/protein-io/badge.svg)](https://docs.rs/protein-io)                                     |
| **protein-analysis** | [![crates.io](https://img.shields.io/crates/d/protein-analysis.svg)](https://crates.io/crates/protein-analysis)[![crates.io](https://img.shields.io/crates/v/protein-analysis.svg)](https://crates.io/crates/protein-analysis)[![docs.rs](https://docs.rs/protein-analysis/badge.svg)](https://docs.rs/protein-analysis) |



## Example

Let's read a protein structure from a PDB file and draw a Ramachandran plot!


```rust
use csv::Writer; // the crate `csv` is required if you want to output csv
use protein::{
    io::pdb::Parser, // the PDB parser that parses PDB file into a `Structure`
    analysis::ModelAnalysis // `Structure` alone only stores data.
                              // Functions for analysing the `Structure` are provided by separate traits
 };
use std::fs;

fn main() {
    let pdbfile = get_pdb("4f7i").unwrap();
    let structure = Parser::parse(&pdbfile).unwrap();
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

```

This will produce a csv file containing two columns representing phi and psi angles. Then we can read the csv file in R and plot it (unfortunately I am not familiar with any graphing libraries in Rust):

![ramachandran plot](./examples/ramachandran.png)

You can directly run the above example using `cargo run`:

```bash
cargo run --example ramachandran
```


<!-- ## IO Formats

PDB is the oldest, and probably the most well-known file format in the field of structural biology. However, [as claimed by RSCB](https://pdb101.rcsb.org/learn/guide-to-understanding-pdb-data/beginner%E2%80%99s-guide-to-pdb-structures-and-the-pdbx-mmcif-format), there are some limitations of the PDB file format and it is expected to be replaced the the PDBx/mmCIF format. Therefore, while this crate provides methods to manipulate PDB files, the PDBx/mmCIF format is our first-class citizen. -->

