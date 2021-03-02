//! Primitives for reading and writing the [Simplified Molecular Line Input Line Entry System](https://en.wikipedia.org/wiki/Simplified_molecular-input_line-entry_system) (SMILES) language. Based on [OpenSMILES](http://opensmiles.org).
//! For goals and rationale, see:
//! 
//! - [SMILES Formal Grammar](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)
//! - [SMILES Formal Grammar Revisited](https://depth-first.com/articles/2020/04/20/smiles-formal-grammar/)
//! - [Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/12/14/an-abstract-syntatx-tree-for-smiles/)
//! - [Abstract Syntax Trees for SMILES](https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/)

/// Common components used in `graph` and `tree` representations.
pub mod feature;
/// Reading SMILES representations from strings.
pub mod read;
/// Writing SMILES string representations.
pub mod write;
/// Traversal of an adjacency representation.
pub mod walk;
/// SMILES adjacency list representation.
pub mod graph;

// https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}