# Purr

Primitives for reading and writing the SMILES language in Rust. For details, see:

- [SMILES Formal Grammar](https://depth-first.com/articles/2020/04/20/smiles-formal-grammar/)
- [SMILES Formal Grammar Revisited](https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/)
- [Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)
- [Abstract Syntax Trees for SMILES](https://depth-first.com/articles/2020/12/14/an-abstract-syntatx-tree-for-smiles/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
purr = "0.8"
```

## Examples

Parse ethanol into an abstract syntax tree:

```rust
use purr::read::{ read, Error };
use purr::tree::{ Atom, Link, Target };
use purr::parts::{
  AtomKind, Aliphatic, BondKind, Element, BracketSymbol, VirtualHydrogen
};

fn main() -> Result<(), Error> {
    let root = read("OC[CH3]", None)?;

    assert_eq!(root, Atom {
        kind: AtomKind::Aliphatic(Aliphatic::O),
        links: vec![
            Link::Bond {
                kind: BondKind::Elided,
                target: Target::Atom(Atom {
                    kind: AtomKind::Aliphatic(Aliphatic::C),
                    links: vec![
                        Link::Bond {
                            kind: BondKind::Elided,
                            target: Target::Atom(Atom {
                                kind: AtomKind::Bracket {
                                    isotope: None,
                                    symbol: BracketSymbol::Element(Element::C),
                                    configuration: None,
                                    hcount: Some(VirtualHydrogen::H3),
                                    charge: None,
                                    map: None
                                },
                                links: vec![ ]
                            })
                        }
                    ]
                })
            }
        ]
    });

    Ok(())
}
```

It's often helpful to represent a tree as a string for visual inspection.

```rust
use purr::read::{ read, Error };
use purr::tree::Writer;

fn main() -> Result<(), Error> {
    let root = read("c1ccccc1", None)?;

    assert_eq!(Writer::write(&root), "c1ccccc1");

    Ok(())
}
```

The `trace` value maps each `Atom` index to a cursor position in the original string. This is useful when conveying semantic errors such as hypervalence. 

```rust
use purr::read::{ read, Trace, Error };

fn main() -> Result<(), Error> {
    let mut trace = Trace::new();
    let _ = read("C=C(C)(C)C", Some(&mut trace))?;

    assert_eq!(trace, Trace {
        atoms: vec![ 0..1, 2..3, 4..5, 7..8, 9..10 ],
        bonds: vec![ 1, 4, 7, 9 ],
        rnums: vec![ ]
    });

    // obtain the cursor position of the atom at index 1 (second atom):
    assert_eq!(trace.atoms[1], 2..3);

    Ok(())
}
```

Syntax errors are mapped to the character index.

```rust
use purr::read::{ read, Error };

fn main() {
    assert_eq!(read("OCCXC", None), Err(Error::InvalidCharacter(3)));
}
```

Sometimes it's more convenient to work with an adjacency (or graph-like) representation. This can be accomplished through the `graph_from_tree` method.

```rust
use purr::read::{ read };
use purr::graph::{ Atom, Bond, from_tree, Error };
use purr::parts::{ AtomKind, Aliphatic, BondKind };

fn main() -> Result<(), Error> {
    let root = read("C=*", None).expect("read");
    let graph = from_tree(root, None)?;
 
    assert_eq!(graph, vec![
        Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Double, 1)
            ]
        },
        Atom {
            kind: AtomKind::Star,
            bonds: vec![
                Bond::new(BondKind::Double, 0)
            ]
        }
     ]);

    assert_eq!(graph[0].is_aromatic(), false);
    assert_eq!(graph[0].subvalence(), 2);
 
    Ok(())
}
```

It may be useful to map specific graph features to the cursor position in the SMILES string that generated them. This is possible using Purr's trace capability.

```rust
use purr::{ graph, read };

fn main() -> Result<(), read::Error> {
    let mut tree_trace = read::Trace::new();
    //           atom ids: 0 12    345
    let tree = read::read("C1C[CH2]CCC=1", Some(&mut tree_trace))?;
    //             cursor: 0123456789012
    let mut graph_trace = graph::Trace::new();
    let _ = graph::from_tree(tree, Some(&mut graph_trace))
        .expect("graph from tree");
    
    let bond_5_0 = graph_trace.bond_id(5, 0).expect("bond 5, 0");
    let bond_5_0_cursor = tree_trace.bonds[bond_5_0];
    
    assert_eq!(bond_5_0_cursor, 11); // bond(5, 0) @ cursor(1), type =

    let bond_0_5 = graph_trace.bond_id(0, 5).expect("bond 0, 5");
    let bond_0_5_cursor = tree_trace.bonds[bond_0_5];

    assert_eq!(bond_0_5_cursor, 1); // bond(0, 5) @ cursor(1), type elided

    let atom_1 = tree_trace.atoms[2].clone(); // atom(2) @ cursor(3..8)

    assert_eq!(atom_1, 3..8);

    Ok(())
}
```

# Versions

Purr is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Purr is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.