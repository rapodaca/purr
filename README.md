# Purr

Primitives for reading and writing the SMILES language in Rust. For details, see:

- [Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)
- [SMILES Formal Grammar](https://depth-first.com/articles/2020/04/20/smiles-formal-grammar/)
- [Abstract Syntax Trees for SMILES](https://depth-first.com/articles/2020/12/14/an-abstract-syntatx-tree-for-smiles/)
- [SMILES Formal Grammar Revisited](https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
purr = "0.7"
```

## Examples

Parse ethanol into an abstract syntax tree:

```rust
use purr::read::{ read, Reading, Error };
use purr::parts::{ AtomKind, Aliphatic, BondKind, Element, BracketSymbol };
use purr::tree::{ Atom, Link, Target };

fn main() -> Result<(), Error> {
    let Reading { root, trace } = read("OC[CH3]")?;

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
                                    parity: None,
                                    hcount: Some(3),
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
    use purr::write::write;

    fn main() -> Result<(), Error> {
        let root = read("c1ccccc1")?.root;

        assert_eq!(write(&root), "c1ccccc1");

        Ok(())
    }
```

The `trace` value maps each `Atom` index to a cursor position in the original string. This is useful when conveying semantic errors such as hypervalence. 

```rust
use purr::read::{ read, Error };

fn main() -> Result<(), Error> {
    let trace = read("C=C(C)(C)C")?.trace;

    assert_eq!(trace, vec![ 0, 2, 4, 7, 9 ]);

    // obtain the cursor position of the atom at index 1 (second atom):
    assert_eq!(trace[1], 2);

    Ok(())
}
```

Syntax errors are mapped to the character index.

```rust
use purr::read::{ read, Error };

fn main() {
    assert_eq!(read("OCCXC"), Err(Error::InvalidCharacter(3)));
}
```

Sometimes it's more convenient to work with an adjacency (or graph-like) representation. This can be accomplished through the `graph::from_tree` method.

```rust
use purr::read::{ read, Error };
use purr::parts::{ AtomKind, BondKind };
use purr::graph::{ Atom, Bond, from_tree };

fn main() -> Result<(), Error> {
    let root = read("*=*")?.root;

    assert_eq!(from_tree(root).expect("semantic error"), vec![
        Atom {
            kind: AtomKind::Star,
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

    Ok(())
}
```

# Versions

Purr is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Purr is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.