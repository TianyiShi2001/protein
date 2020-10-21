# protein-get

The **protein-get** crate provides simple functions to fetch protein structure data files in `.pdb`, `.cif` and `.bcif` formats
from [**RSCB**](https://rcsb.org).

## Example

```rust
use protein_get::get_pdb;

fn main(){
    let pdb1a8o = get_pdb("1A8O").unwrap();
    assert_eq!(&pdb1a8o[62..66], b"1A8O");
}
```