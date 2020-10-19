//! # protein-core
//!
//! This crate aims to provide a unified framework for representing protein structural data in Rust.
//!
//! ## `Structure`
//!
//! The central struct provided by this crate, [`Structure`], hold information roughly equivalent to that contained in a PDB or mmCIF file.
//! A minimal [`Structure`] contains data that unambiguously describe the primary structure (sequences of nucleotides and/or amino acids),
//! any secondary structure (helices and sheets) and the identity and coordinates and connectivity of all atoms in the structure assembly.
//! It can also include [`Metadata`] as an `Option`.
//!
//! A `Structure` can either be created from scratch or parsed from specialized file formats such as PDB and mmCIF. All parsers
//! should aim to parse information into structs provided in the [`structure`] and [`metadata`] module.
//!
//! ## `Metadata`
//!
//! [`Metadata`] are data that are not strictly required for describing the structure, such as the title, author, and experimental method.
//! Each field in [`Metadata`] is `Option`al.
//!
//! [`Structure`]: structure/struct.Structure.html
//! [`structure`]: structure/
//! [`Metadata`]: metadata/struct.Metadata.html
//! [`metadata`]: metadata/

pub mod structure;
pub use structure::Structure;
pub mod metadata;
pub use metadata::Metadata;

pub mod data;
