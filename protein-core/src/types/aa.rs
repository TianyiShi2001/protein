use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
    Mse,
    Pyl, // https://www.wwpdb.org/news/news?year=2014#5764490799cccf749a90cddf
    Sec, // https://www.wwpdb.org/news/news?year=2014#5764490799cccf749a90cddf
    Other(usize),
    Nonstandard(NonstandardAminoAcid),
    Custom(String),
}

impl FromStr for AminoAcid {
    type Err = String;
    fn from_str(inp: &str) -> Result<Self, <Self as FromStr>::Err> {
        match inp {
            "ALA" => Ok(Self::Ala),
            "ARG" => Ok(Self::Arg),
            "ASN" => Ok(Self::Asn),
            "ASP" => Ok(Self::Asp),
            "CYS" => Ok(Self::Cys),
            "GLN" => Ok(Self::Gln),
            "GLU" => Ok(Self::Glu),
            "GLY" => Ok(Self::Gly),
            "HIS" => Ok(Self::His),
            "ILE" => Ok(Self::Ile),
            "LEU" => Ok(Self::Leu),
            "LYS" => Ok(Self::Lys),
            "MET" => Ok(Self::Met),
            "PHE" => Ok(Self::Phe),
            "PRO" => Ok(Self::Pro),
            "SER" => Ok(Self::Ser),
            "THR" => Ok(Self::Thr),
            "TRP" => Ok(Self::Trp),
            "TYR" => Ok(Self::Tyr),
            "VAL" => Ok(Self::Val),
            "PYL" => Ok(Self::Pyl),
            "SEC" => Ok(Self::Sec),
            _ => Err("Not a standard amino acid!".to_string()),
        }
    }
}

impl AminoAcid {
    pub fn parse(inp: &str) -> Self {
        if let Ok(aa) = Self::from_str(inp) {
            aa
        } else if let Ok(aa) = NonstandardAminoAcid::from_str(inp) {
            Self::Nonstandard(aa)
        } else {
            Self::Custom(inp.to_owned())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NonstandardAminoAcid {
    Dm0,
    Api,
    Fty,
    Oar,
    Thr,
    Dbb,
    Trn,
    Fgl,
    Bhd,
    Aln,
    Gma,
    Pha,
    Dnp,
    Dgn,
    Dar,
    Dva,
    Dpr,
    Dsn,
    Aho,
    Cyq,
    Gpl,
    Pm3,
    Set,
    Tyy,
    Ar7,
    Csa,
    Pbf,
    Csh,
    Psa,
    Csz,
    Tro,
    B2A,
    Ble,
    B2F,
    Lyn,
    Mme,
    Sui,
    Omy,
    Phl,
    Icy,
    Hso,
    Alv,
    Mle,
    Ahp,
    Hse,
    Hip,
    Niy,
    Iil,
    Crg,
    Lyx,
    Bal,
    Trw,
    Fgp,
    Dmk,
    Dmh,
    Ahb,
    Hti,
    Smf,
    B2V,
    Ncb,
    Bmt,
    Yof,
    Cr0,
    Nal,
    Dbz,
    Paq,
    Dde,
    Fva,
    Dty,
    Fho,
    Crk,
    Dyg,
    Iam,
    Crx,
    Dly,
    Bif,
    Dhi,
    Cr7,
    Cyr,
    Cmh,
    Hy3,
    Qlg,
    Sc2,
    Mcs,
    Sxe,
    Svy,
    Pf5,
    B27,
    Nyg,
    Kgc,
    B3E,
    B3Y,
    N10,
    Dgl,
    Dil,
    Dth,
    Dsg,
    Dcy,
    Kst,
    Cml,
    Ty2,
    Th5,
    Lp6,
    Psw,
    Ynm,
    Na8,
    Th6,
    Fc0,
    Izo,
    Swg,
    Ht7,
    Hr7,
    Cfy,
    La2,
    Le1,
    Xsn,
    Mp8,
    Wfp,
    Nrp,
    Pr4,
    Ty8,
    Ty9,
    Me0,
    Ty5,
    Re0,
    Jjj,
    Xcn,
    Met,
    Ecc,
    Wcr,
    Kws,
    Plj,
    Qfg,
    Qcs,
    Cq1,
    Cq2,
    U2X,
    Uox,
    Fak,
    C1S,
    Ofm,
    Byr,
    L5P,
    Lrk,
    N80,
    Lyr,
    Dya,
    Nzc,
    P1L,
    Nlw,
    Vya,
    B2H,
    Pdf,
    F2Y,
    Eja,
    Tnq,
    Snm,
    His,
    Jby,
    Kzv,
    Kz7,
    Kz4,
    Kzg,
    Ky4,
    Kz1,
    Ky7,
    Kxv,
    Kzy,
    Qc4,
    Ff9,
}
impl NonstandardAminoAcid {
    pub fn standard_res(&self) -> Option<AminoAcid> {
        match &self {
            Self::Dm0 => Some(AminoAcid::Lys),
            Self::Api => Some(AminoAcid::Lys),
            Self::Fty => Some(AminoAcid::Tyr),
            Self::Oar => Some(AminoAcid::Arg),
            Self::Thr => Some(AminoAcid::Thr),
            Self::Dbb => Some(AminoAcid::Thr),
            Self::Trn => Some(AminoAcid::Trp),
            Self::Fgl => Some(AminoAcid::Gly),
            Self::Bhd => Some(AminoAcid::Asp),
            Self::Aln => Some(AminoAcid::Ala),
            Self::Gma => Some(AminoAcid::Glu),
            Self::Pha => Some(AminoAcid::Phe),
            Self::Dnp => Some(AminoAcid::Ala),
            Self::Dgn => Some(AminoAcid::Gln),
            Self::Dar => Some(AminoAcid::Arg),
            Self::Dva => Some(AminoAcid::Val),
            Self::Dpr => Some(AminoAcid::Pro),
            Self::Dsn => Some(AminoAcid::Ser),
            Self::Aho => Some(AminoAcid::Ala),
            Self::Cyq => Some(AminoAcid::Cys),
            Self::Gpl => Some(AminoAcid::Lys),
            Self::Pm3 => Some(AminoAcid::Phe),
            Self::Set => Some(AminoAcid::Ser),
            Self::Tyy => Some(AminoAcid::Tyr),
            Self::Ar7 => Some(AminoAcid::Arg),
            Self::Csa => Some(AminoAcid::Cys),
            Self::Pbf => Some(AminoAcid::Phe),
            Self::Csh => Some(AminoAcid::Ser),
            Self::Psa => Some(AminoAcid::Phe),
            Self::Csz => Some(AminoAcid::Cys),
            Self::Tro => Some(AminoAcid::Trp),
            Self::B2A => Some(AminoAcid::Ala),
            Self::Ble => Some(AminoAcid::Leu),
            Self::B2F => Some(AminoAcid::Phe),
            Self::Lyn => Some(AminoAcid::Lys),
            Self::Mme => Some(AminoAcid::Met),
            Self::Sui => Some(AminoAcid::Asp),
            Self::Omy => Some(AminoAcid::Tyr),
            Self::Phl => Some(AminoAcid::Phe),
            Self::Icy => Some(AminoAcid::Cys),
            Self::Hso => Some(AminoAcid::His),
            Self::Alv => Some(AminoAcid::Ala),
            Self::Mle => Some(AminoAcid::Leu),
            Self::Ahp => Some(AminoAcid::Ala),
            Self::Hse => Some(AminoAcid::Ser),
            Self::Hip => Some(AminoAcid::His),
            Self::Niy => Some(AminoAcid::Tyr),
            Self::Iil => Some(AminoAcid::Ile),
            Self::Crg => Some(AminoAcid::Thr),
            Self::Lyx => Some(AminoAcid::Lys),
            Self::Bal => Some(AminoAcid::Ala),
            Self::Trw => Some(AminoAcid::Trp),
            Self::Fgp => Some(AminoAcid::Ser),
            Self::Dmk => Some(AminoAcid::Asp),
            Self::Dmh => Some(AminoAcid::Asn),
            Self::Ahb => Some(AminoAcid::Asn),
            Self::Hti => Some(AminoAcid::Cys),
            Self::Smf => Some(AminoAcid::Phe),
            Self::B2V => Some(AminoAcid::Val),
            Self::Ncb => Some(AminoAcid::Ala),
            Self::Bmt => Some(AminoAcid::Thr),
            Self::Yof => Some(AminoAcid::Tyr),
            Self::Cr0 => Some(AminoAcid::Thr),
            Self::Nal => Some(AminoAcid::Ala),
            Self::Dbz => Some(AminoAcid::Ala),
            Self::Paq => Some(AminoAcid::Tyr),
            Self::Dde => Some(AminoAcid::His),
            Self::Fva => Some(AminoAcid::Val),
            Self::Dty => Some(AminoAcid::Tyr),
            Self::Fho => Some(AminoAcid::Lys),
            Self::Crk => Some(AminoAcid::Met),
            Self::Dyg => Some(AminoAcid::Asp),
            Self::Iam => Some(AminoAcid::Ala),
            Self::Crx => Some(AminoAcid::Ala),
            Self::Dly => Some(AminoAcid::Lys),
            Self::Bif => Some(AminoAcid::Phe),
            Self::Dhi => Some(AminoAcid::His),
            Self::Cr7 => Some(AminoAcid::Lys),
            Self::Cyr => Some(AminoAcid::Cys),
            Self::Cmh => Some(AminoAcid::Cys),
            Self::Hy3 => Some(AminoAcid::Pro),
            Self::Qlg => Some(AminoAcid::Gln),
            Self::Sc2 => Some(AminoAcid::Cys),
            Self::Mcs => Some(AminoAcid::Cys),
            Self::Sxe => Some(AminoAcid::Ser),
            Self::Svy => Some(AminoAcid::Ser),
            Self::Pf5 => Some(AminoAcid::Phe),
            Self::B27 => Some(AminoAcid::Thr),
            Self::Nyg => Some(AminoAcid::Asn),
            Self::Kgc => Some(AminoAcid::Lys),
            Self::B3E => Some(AminoAcid::Glu),
            Self::B3Y => Some(AminoAcid::Tyr),
            Self::N10 => Some(AminoAcid::Ser),
            Self::Dgl => Some(AminoAcid::Glu),
            Self::Dil => Some(AminoAcid::Ile),
            Self::Dth => Some(AminoAcid::Thr),
            Self::Dsg => Some(AminoAcid::Asn),
            Self::Dcy => Some(AminoAcid::Cys),
            Self::Kst => Some(AminoAcid::Lys),
            Self::Cml => Some(AminoAcid::Cys),
            Self::Ty2 => Some(AminoAcid::Tyr),
            Self::Th5 => Some(AminoAcid::Thr),
            Self::Lp6 => Some(AminoAcid::Lys),
            Self::Psw => Some(AminoAcid::Sec),
            Self::Ynm => Some(AminoAcid::Tyr),
            Self::Na8 => Some(AminoAcid::Ala),
            Self::Th6 => Some(AminoAcid::Thr),
            Self::Fc0 => Some(AminoAcid::Phe),
            Self::Izo => Some(AminoAcid::Met),
            Self::Swg => Some(AminoAcid::Ser),
            Self::Ht7 => Some(AminoAcid::Trp),
            Self::Hr7 => Some(AminoAcid::Arg),
            Self::Cfy => Some(AminoAcid::Phe),
            Self::La2 => Some(AminoAcid::Lys),
            Self::Le1 => Some(AminoAcid::Val),
            Self::Xsn => Some(AminoAcid::Asn),
            Self::Mp8 => Some(AminoAcid::Pro),
            Self::Wfp => Some(AminoAcid::Phe),
            Self::Nrp => Some(AminoAcid::Leu),
            Self::Pr4 => Some(AminoAcid::Pro),
            Self::Ty8 => Some(AminoAcid::Tyr),
            Self::Ty9 => Some(AminoAcid::Tyr),
            Self::Me0 => Some(AminoAcid::Met),
            Self::Ty5 => Some(AminoAcid::Tyr),
            Self::Re0 => Some(AminoAcid::Trp),
            Self::Jjj => Some(AminoAcid::Cys),
            Self::Xcn => Some(AminoAcid::Cys),
            Self::Met => Some(AminoAcid::Gln),
            Self::Ecc => Some(AminoAcid::Gln),
            Self::Wcr => Some(AminoAcid::Gly),
            Self::Kws => Some(AminoAcid::Thr),
            Self::Plj => Some(AminoAcid::Pro),
            Self::Qfg => Some(AminoAcid::Gln),
            Self::Qcs => Some(AminoAcid::Cys),
            Self::Cq1 => Some(AminoAcid::Thr),
            Self::Cq2 => Some(AminoAcid::Gly),
            Self::U2X => Some(AminoAcid::Tyr),
            Self::Uox => Some(AminoAcid::Sec),
            Self::Fak => Some(AminoAcid::Lys),
            Self::C1S => Some(AminoAcid::Cys),
            Self::Ofm => Some(AminoAcid::Phe),
            Self::Byr => Some(AminoAcid::Tyr),
            Self::L5P => Some(AminoAcid::Lys),
            Self::Lrk => Some(AminoAcid::Lys),
            Self::N80 => Some(AminoAcid::Pro),
            Self::Lyr => Some(AminoAcid::Lys),
            Self::Dya => Some(AminoAcid::Asp),
            Self::Nzc => Some(AminoAcid::Thr),
            Self::P1L => Some(AminoAcid::Cys),
            Self::Nlw => Some(AminoAcid::Leu),
            Self::Vya => Some(AminoAcid::Gly),
            Self::B2H => Some(AminoAcid::Gly),
            Self::Pdf => Some(AminoAcid::Pro),
            Self::F2Y => Some(AminoAcid::Tyr),
            Self::Eja => Some(AminoAcid::Cys),
            Self::Tnq => Some(AminoAcid::Trp),
            Self::Snm => Some(AminoAcid::Ser),
            Self::His => Some(AminoAcid::His),
            Self::Jby => Some(AminoAcid::Gly),
            Self::Kzv => Some(AminoAcid::Cys),
            Self::Kz7 => Some(AminoAcid::Cys),
            Self::Kz4 => Some(AminoAcid::Cys),
            Self::Kzg => Some(AminoAcid::Cys),
            Self::Ky4 => Some(AminoAcid::Cys),
            Self::Kz1 => Some(AminoAcid::Cys),
            Self::Ky7 => Some(AminoAcid::Cys),
            Self::Kxv => Some(AminoAcid::Cys),
            Self::Kzy => Some(AminoAcid::Cys),
            Self::Qc4 => Some(AminoAcid::Thr),
            Self::Ff9 => Some(AminoAcid::Lys),
        }
    }
    pub fn description(&self) -> &'static str {
        match &self {
            Self::Dm0 => "",
            Self::Api => "2,6-DIAMINOPIMELIC ACID",
            Self::Fty => "DEOXY-DIFLUOROMETHELENE-PHOSPHOTYROSINE",
            Self::Oar => "N-(4-AMINO-5-HYDROXY-PENTYL)-GUANIDINE",
            Self::Thr => "GLYCOSYLATION SITE",
            Self::Dbb => "POST-TRANSLATIONAL MODIFICATION",
            Self::Trn => "NZ2-TRYPTOPHAN",
            Self::Fgl => "2-AMINOPROPANEDIOIC ACID",
            Self::Bhd => "(3S)-3-HYDROXY-L-ASPARTIC ACID",
            Self::Aln => "NAPHTHALEN-2-YL-3-ALANINE",
            Self::Gma => "4-AMIDO-4-CARBAMOYL-BUTYRIC ACID",
            Self::Pha => "PHENYLALANINAL",
            Self::Dnp => "3-AMINO-ALANINE",
            Self::Dgn => "D-GLUTAMINE",
            Self::Dar => "D-ARGININE",
            Self::Dva => "D-VALINE",
            Self::Dpr => "D-PROLINE",
            Self::Dsn => "D-SERINE",
            Self::Aho => "N-ACETYL-N-HYDROXY-L-ORNITHINE",
            Self::Cyq => "",
            Self::Gpl => "LYSINE GUANOSINE-5'-MONOPHOSPHATE",
            Self::Pm3 => "",
            Self::Set => "AMINOSERINE",
            Self::Tyy => "",
            Self::Ar7 => "",
            Self::Csa => "S-ACETONYLCYSTEINE",
            Self::Pbf => "PARA-(BENZOYL)-PHENYLALANINE",
            Self::Csh => "",
            Self::Psa => "",
            Self::Csz => "S-SELANYL CYSTEINE",
            Self::Tro => "2-HYDROXY-TRYPTOPHAN",
            Self::B2A => "ALANINE BORONIC ACID",
            Self::Ble => "LEUCINE BORONIC ACID",
            Self::B2F => "PHENYLALANINE BORONIC ACID",
            Self::Lyn => "2,6-DIAMINO-HEXANOIC ACID AMIDE",
            Self::Mme => "N-METHYL METHIONINE",
            Self::Sui => "",
            Self::Omy => "",
            Self::Phl => "L-PHENYLALANINOL",
            Self::Icy => "S-(2-IODOBENZYL)-L-CYSTEINE",
            Self::Hso => "L-HISTIDINOL",
            Self::Alv => "(2S)-2-AMINOPROPANE-1,1-DIOL",
            Self::Mle => "N-METHYLLEUCINE",
            Self::Ahp => "2-AMINO-HEPTANOIC ACID",
            Self::Hse => "L-HOMOSERINE",
            Self::Hip => "ND1-PHOSPHONOHISTIDINE",
            Self::Niy => "META-NITRO-TYROSINE",
            Self::Iil => "ISO-ISOLEUCINE",
            Self::Crg => "",
            Self::Lyx => "N''-(2-COENZYME A)-PROPANOYL-LYSINE",
            Self::Bal => "BETA-ALANINE",
            Self::Trw => "",
            Self::Fgp => "",
            Self::Dmk => "3,3-DIMETHYL ASPARTIC ACID",
            Self::Dmh => "N4,N4-DIMETHYL-ASPARAGINE",
            Self::Ahb => "BETA-HYDROXYASPARAGINE",
            Self::Hti => "",
            Self::Smf => "4-SULFOMETHYL-L-PHENYLALANINE",
            Self::B2V => "VALINE BORONIC ACID",
            Self::Ncb => "N-CARBAMOYL-ALANINE",
            Self::Bmt => "",
            Self::Yof => "3-FLUOROTYROSINE",
            Self::Cr0 => "",
            Self::Nal => "BETA-(2-NAPHTHYL)-ALANINE",
            Self::Dbz => "3-(BENZOYLAMINO)-L-ALANINE",
            Self::Paq => "",
            Self::Dde => "",
            Self::Fva => "N-FORMYL-L-VALINE",
            Self::Dty => "D-TYROSINE",
            Self::Fho => "N^5^-FORMYL-N^5^-HYDROXY-L-ORNITHINE",
            Self::Crk => "",
            Self::Dyg => "",
            Self::Iam => "4-[(ISOPROPYLAMINO)METHYL]PHENYLALANINE",
            Self::Crx => "",
            Self::Dly => "D-LYSINE",
            Self::Bif => "",
            Self::Dhi => "D-HISTIDINE",
            Self::Cr7 => "",
            Self::Cyr => "",
            Self::Cmh => "S-(METHYLMERCURY)-L-CYSTEINE",
            Self::Hy3 => "3-HYDROXYPROLINE",
            Self::Qlg => "",
            Self::Sc2 => "N-ACETYL-L-CYSTEINE",
            Self::Mcs => "MALONYL CYSTEINE",
            Self::Sxe => "",
            Self::Svy => "",
            Self::Pf5 => "2,3,4,5,6-PENTAFLUORO-L-PHENYLALANINE",
            Self::B27 => "(2R,3S) 3-AMINO-4-MERCAPTO-2-BUTANOL",
            Self::Nyg => "",
            Self::Kgc => "",
            Self::B3E => "(3S)-3-AMINOHEXANEDIOIC ACID",
            Self::B3Y => "",
            Self::N10 => "O-[(HEXYLAMINO)CARBONYL]-L-SERINE",
            Self::Dgl => "D-GLUTAMIC ACID",
            Self::Dil => "D-ISOLEUCINE",
            Self::Dth => "D-THREONINE",
            Self::Dsg => "D-ASPARAGINE",
            Self::Dcy => "D-CYSTEINE",
            Self::Kst => "N~6~-(5-CARBOXY-3-THIENYL)-L-LYSINE",
            Self::Cml => "",
            Self::Ty2 => "3-AMINO-L-TYROSINE",
            Self::Th5 => "O-ACETYL-L-THREONINE",
            Self::Lp6 => "6-PIPERIDIN-1-YL-L-NORLEUCINE",
            Self::Psw => "3-(SULFANYLSELANYL)-L-ALANINE",
            Self::Ynm => "N-METHYL-L-TYROSINE",
            Self::Na8 => "BETA-(2-NAPHTHYL)-ALANINE",
            Self::Th6 => "4-HYDROXY-L-THREONINE",
            Self::Fc0 => "N-CARBOXY-L-PHENYLALANINE",
            Self::Izo => "(2S)-2-AMINOHEX-5-YNOIC ACID",
            Self::Swg => "",
            Self::Ht7 => "",
            Self::Hr7 => "",
            Self::Cfy => "",
            Self::La2 => "",
            Self::Le1 => "3-SULFANYL-L-VALINE",
            Self::Xsn => "L-ALPHA-ASPARAGINE",
            Self::Mp8 => "(4R)-4-METHYL-L-PROLINE",
            Self::Wfp => "3,5-DIFLUORO-L-PHENYLALANINE",
            Self::Nrp => "",
            Self::Pr4 => "",
            Self::Ty8 => "",
            Self::Ty9 => "",
            Self::Me0 => "HYDROXY-L-METHIONINE",
            Self::Ty5 => "O-BENZYL-L-TYROSINE",
            Self::Re0 => "",
            Self::Jjj => "S-(PYRIDIN-3-YLCARBONYL)-L-CYSTEINE",
            Self::Xcn => "S-CYANO-L-CYSTEINE",
            Self::Met => "CHROMOPHORE",
            Self::Ecc => "(4S)-4-AMINO-5-HYDROXYPENTANAMIDE",
            Self::Wcr => "",
            Self::Kws => "",
            Self::Plj => "METHYL L-PROLINATE",
            Self::Qfg => "",
            Self::Qcs => "S-CARBAMOYL-L-CYSTEINE",
            Self::Cq1 => "",
            Self::Cq2 => "",
            Self::U2X => "O-(CYCLOHEXYLMETHYL)-L-TYROSINE",
            Self::Uox => "3-(OXIDO-LAMBDA~4~-SELANYL)-L-ALANINE",
            Self::Fak => "N~6~-(TRIFLUOROACETYL)-L-LYSINE",
            Self::C1S => "3-(PROP-2-EN-1-YLDISULFANYL)-L-ALANINE",
            Self::Ofm => "CIRCULARIZED CHROMOPHORE",
            Self::Byr => "MODIFIED RESIDUE",
            Self::L5P => "MODIFIED RESIDUE",
            Self::Lrk => "MODIFIED RESIDUE",
            Self::N80 => "MODIFIED RESIDUE",
            Self::Lyr => "MODIFIED RESIDUE",
            Self::Dya => "DIDEHYDROASPARTATE",
            Self::Nzc => "N-METHYLIDENE-L-THREONINE",
            Self::P1L => "MODIFIED RESIDUE",
            Self::Nlw => "MODIFIED RESIDUE",
            Self::Vya => "CHROMOPHORE",
            Self::B2H => "CHROMOPHORE",
            Self::Pdf => "MODIFIED RESIDUE",
            Self::F2Y => "MODIFIED RESIDUE",
            Self::Eja => "MODIFIED RESIDUE",
            Self::Tnq => "MODIFIED RESIDUE",
            Self::Snm => "MODIFIED RESIDUE",
            Self::His => "MODIFIED RESIDUE",
            Self::Jby => "CHROMOPHORE",
            Self::Kzv => "CHROMOPHORE",
            Self::Kz7 => "CHROMOPHORE",
            Self::Kz4 => "CHROMOPHORE",
            Self::Kzg => "CHROMOPHORE",
            Self::Ky4 => "CHROMOPHORE",
            Self::Kz1 => "CHROMOPHORE",
            Self::Ky7 => "CHROMOPHORE",
            Self::Kxv => "CHROMOPHORE",
            Self::Kzy => "CHROMOPHORE",
            Self::Qc4 => "CHROMOPHORE",
            Self::Ff9 => "MODIFIED RESIDUE",
        }
    }
}
impl FromStr for NonstandardAminoAcid {
    type Err = String;
    fn from_str(inp: &str) -> Result<Self, <Self as FromStr>::Err> {
        match inp {
            "DM0" => Ok(Self::Dm0),
            "API" => Ok(Self::Api),
            "FTY" => Ok(Self::Fty),
            "OAR" => Ok(Self::Oar),
            "THR" => Ok(Self::Thr),
            "DBB" => Ok(Self::Dbb),
            "TRN" => Ok(Self::Trn),
            "FGL" => Ok(Self::Fgl),
            "BHD" => Ok(Self::Bhd),
            "ALN" => Ok(Self::Aln),
            "GMA" => Ok(Self::Gma),
            "PHA" => Ok(Self::Pha),
            "DNP" => Ok(Self::Dnp),
            "DGN" => Ok(Self::Dgn),
            "DAR" => Ok(Self::Dar),
            "DVA" => Ok(Self::Dva),
            "DPR" => Ok(Self::Dpr),
            "DSN" => Ok(Self::Dsn),
            "AHO" => Ok(Self::Aho),
            "CYQ" => Ok(Self::Cyq),
            "GPL" => Ok(Self::Gpl),
            "PM3" => Ok(Self::Pm3),
            "SET" => Ok(Self::Set),
            "TYY" => Ok(Self::Tyy),
            "AR7" => Ok(Self::Ar7),
            "CSA" => Ok(Self::Csa),
            "PBF" => Ok(Self::Pbf),
            "CSH" => Ok(Self::Csh),
            "PSA" => Ok(Self::Psa),
            "CSZ" => Ok(Self::Csz),
            "TRO" => Ok(Self::Tro),
            "B2A" => Ok(Self::B2A),
            "BLE" => Ok(Self::Ble),
            "B2F" => Ok(Self::B2F),
            "LYN" => Ok(Self::Lyn),
            "MME" => Ok(Self::Mme),
            "SUI" => Ok(Self::Sui),
            "OMY" => Ok(Self::Omy),
            "PHL" => Ok(Self::Phl),
            "ICY" => Ok(Self::Icy),
            "HSO" => Ok(Self::Hso),
            "ALV" => Ok(Self::Alv),
            "MLE" => Ok(Self::Mle),
            "AHP" => Ok(Self::Ahp),
            "HSE" => Ok(Self::Hse),
            "HIP" => Ok(Self::Hip),
            "NIY" => Ok(Self::Niy),
            "IIL" => Ok(Self::Iil),
            "CRG" => Ok(Self::Crg),
            "LYX" => Ok(Self::Lyx),
            "BAL" => Ok(Self::Bal),
            "TRW" => Ok(Self::Trw),
            "FGP" => Ok(Self::Fgp),
            "DMK" => Ok(Self::Dmk),
            "DMH" => Ok(Self::Dmh),
            "AHB" => Ok(Self::Ahb),
            "HTI" => Ok(Self::Hti),
            "SMF" => Ok(Self::Smf),
            "B2V" => Ok(Self::B2V),
            "NCB" => Ok(Self::Ncb),
            "BMT" => Ok(Self::Bmt),
            "YOF" => Ok(Self::Yof),
            "CR0" => Ok(Self::Cr0),
            "NAL" => Ok(Self::Nal),
            "DBZ" => Ok(Self::Dbz),
            "PAQ" => Ok(Self::Paq),
            "DDE" => Ok(Self::Dde),
            "FVA" => Ok(Self::Fva),
            "DTY" => Ok(Self::Dty),
            "FHO" => Ok(Self::Fho),
            "CRK" => Ok(Self::Crk),
            "DYG" => Ok(Self::Dyg),
            "IAM" => Ok(Self::Iam),
            "CRX" => Ok(Self::Crx),
            "DLY" => Ok(Self::Dly),
            "BIF" => Ok(Self::Bif),
            "DHI" => Ok(Self::Dhi),
            "CR7" => Ok(Self::Cr7),
            "CYR" => Ok(Self::Cyr),
            "CMH" => Ok(Self::Cmh),
            "HY3" => Ok(Self::Hy3),
            "QLG" => Ok(Self::Qlg),
            "SC2" => Ok(Self::Sc2),
            "MCS" => Ok(Self::Mcs),
            "SXE" => Ok(Self::Sxe),
            "SVY" => Ok(Self::Svy),
            "PF5" => Ok(Self::Pf5),
            "B27" => Ok(Self::B27),
            "NYG" => Ok(Self::Nyg),
            "KGC" => Ok(Self::Kgc),
            "B3E" => Ok(Self::B3E),
            "B3Y" => Ok(Self::B3Y),
            "N10" => Ok(Self::N10),
            "DGL" => Ok(Self::Dgl),
            "DIL" => Ok(Self::Dil),
            "DTH" => Ok(Self::Dth),
            "DSG" => Ok(Self::Dsg),
            "DCY" => Ok(Self::Dcy),
            "KST" => Ok(Self::Kst),
            "CML" => Ok(Self::Cml),
            "TY2" => Ok(Self::Ty2),
            "TH5" => Ok(Self::Th5),
            "LP6" => Ok(Self::Lp6),
            "PSW" => Ok(Self::Psw),
            "YNM" => Ok(Self::Ynm),
            "NA8" => Ok(Self::Na8),
            "TH6" => Ok(Self::Th6),
            "FC0" => Ok(Self::Fc0),
            "IZO" => Ok(Self::Izo),
            "SWG" => Ok(Self::Swg),
            "HT7" => Ok(Self::Ht7),
            "HR7" => Ok(Self::Hr7),
            "CFY" => Ok(Self::Cfy),
            "LA2" => Ok(Self::La2),
            "LE1" => Ok(Self::Le1),
            "XSN" => Ok(Self::Xsn),
            "MP8" => Ok(Self::Mp8),
            "WFP" => Ok(Self::Wfp),
            "NRP" => Ok(Self::Nrp),
            "PR4" => Ok(Self::Pr4),
            "TY8" => Ok(Self::Ty8),
            "TY9" => Ok(Self::Ty9),
            "ME0" => Ok(Self::Me0),
            "TY5" => Ok(Self::Ty5),
            "RE0" => Ok(Self::Re0),
            "JJJ" => Ok(Self::Jjj),
            "XCN" => Ok(Self::Xcn),
            "MET" => Ok(Self::Met),
            "ECC" => Ok(Self::Ecc),
            "WCR" => Ok(Self::Wcr),
            "KWS" => Ok(Self::Kws),
            "PLJ" => Ok(Self::Plj),
            "QFG" => Ok(Self::Qfg),
            "QCS" => Ok(Self::Qcs),
            "CQ1" => Ok(Self::Cq1),
            "CQ2" => Ok(Self::Cq2),
            "U2X" => Ok(Self::U2X),
            "UOX" => Ok(Self::Uox),
            "FAK" => Ok(Self::Fak),
            "C1S" => Ok(Self::C1S),
            "OFM" => Ok(Self::Ofm),
            "BYR" => Ok(Self::Byr),
            "L5P" => Ok(Self::L5P),
            "LRK" => Ok(Self::Lrk),
            "N80" => Ok(Self::N80),
            "LYR" => Ok(Self::Lyr),
            "DYA" => Ok(Self::Dya),
            "NZC" => Ok(Self::Nzc),
            "P1L" => Ok(Self::P1L),
            "NLW" => Ok(Self::Nlw),
            "VYA" => Ok(Self::Vya),
            "B2H" => Ok(Self::B2H),
            "PDF" => Ok(Self::Pdf),
            "F2Y" => Ok(Self::F2Y),
            "EJA" => Ok(Self::Eja),
            "TNQ" => Ok(Self::Tnq),
            "SNM" => Ok(Self::Snm),
            "HIS" => Ok(Self::His),
            "JBY" => Ok(Self::Jby),
            "KZV" => Ok(Self::Kzv),
            "KZ7" => Ok(Self::Kz7),
            "KZ4" => Ok(Self::Kz4),
            "KZG" => Ok(Self::Kzg),
            "KY4" => Ok(Self::Ky4),
            "KZ1" => Ok(Self::Kz1),
            "KY7" => Ok(Self::Ky7),
            "KXV" => Ok(Self::Kxv),
            "KZY" => Ok(Self::Kzy),
            "QC4" => Ok(Self::Qc4),
            "FF9" => Ok(Self::Ff9),
            _ => Err("not a known non-standard amino acid".to_owned()),
        }
    }
}
