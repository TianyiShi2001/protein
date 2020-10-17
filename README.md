# protein

Structural Biology in Rust

## IO Formats

PDB is the oldest, and probably the most well-known file format in the field of structural biology. However, [as claimed by RSCB](https://pdb101.rcsb.org/learn/guide-to-understanding-pdb-data/beginner%E2%80%99s-guide-to-pdb-structures-and-the-pdbx-mmcif-format), there are some limitations of the PDB file format and it is expected to be replaced the the PDBx/mmCIF format. Therefore, while this crate provides methods to manipulate PDB files, the PDBx/mmCIF format is our first-class citizen.

