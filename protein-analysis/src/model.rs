use dihedral::dihedral;
use protein_core::structure::{Model, Residue};

pub trait ModelAnalysis {
    fn ramachandran(&self) -> (Vec<f32>, Vec<f32>);
}

impl ModelAnalysis for Model {
    fn ramachandran(&self) -> (Vec<f32>, Vec<f32>) {
        let mut n: Vec<[f32; 3]> = Default::default();
        let mut ca: Vec<[f32; 3]> = Default::default();
        let mut c: Vec<[f32; 3]> = Default::default();

        let mut i = 0;
        while i < self.atoms.len() {
            if let Residue::AminoAcid(_) = self.atoms[i].residue {
                if self.atoms[i].name.0 == b"N   "[..]// .is_n()
                    && self.atoms[i + 1].name.0 == b"CA  "[..] // .is_ca()
                    && self.atoms[i + 2].name.0 == b"C   "[..]
                // .is_c()
                {
                    n.push(self.atoms[i].coord);
                    ca.push(self.atoms[i + 1].coord);
                    c.push(self.atoms[i + 2].coord);
                    i += 3;
                    continue;
                }
            }
            i += 1;
        }
        assert!(n.len() == ca.len() && n.len() == c.len());
        let mut phis = Vec::new();
        let mut psis = Vec::new();
        for i in 1..n.len() - 1 {
            let phi = dihedral(&[c[i - 1], n[i], ca[i], c[i]]);
            let psi = dihedral(&[n[i], ca[i], c[i], n[i + 1]]);
            phis.push(phi.to_degrees());
            psis.push(psi.to_degrees());
        }
        (phis, psis)
    }
}
