# Purr

Primitives for reading the SMILES language in Rust. For details, see: *[Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)*.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
purr = "0.3"
```

## Examples

Parse ethanol:

```rust
use purr::read::read;
use purr::valence::implicit_hydrogens;
use purr::mol::{ Mol, Atom, Bond, Element };

fn main() {
    let mol = read(&"OC[CH3]").unwrap();

    assert_eq!(mol, Mol {
        atoms: vec![
            Atom { element: Element::O, ..Default::default() },
            Atom { ..Default::default() },
            Atom { hcount: Some(3), ..Default::default() }
        ],
        bonds: vec![
            vec![ Bond { tid: 1, style: None } ],
            vec![ Bond { tid: 0, style: None }, Bond { tid: 2, style: None } ],
            vec![ Bond { tid: 1, style: None } ]
        ]
    });
    assert_eq!(implicit_hydrogens(&0, &mol), Ok(Some(1)));
    assert_eq!(implicit_hydrogens(&1, &mol), Ok(Some(2)));
    assert_eq!(implicit_hydrogens(&2, &mol), Ok(None));
}
```

# Versions

Purr is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Purr is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.