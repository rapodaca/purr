# Purr

Primitives for reading the SMILES language in Rust. For details, see: *[Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)*.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
purr = "0.1"
```

## Examples

Parse ethane:

```rust
use purr::{ smiles_to_mol, Mol, Atom, Bond, Element };

fn main() {
    let mol = smiles_to_mol(&"CC").unwrap();

    assert_eq!(mol, Mol {
        atoms: vec![
            Atom { element: Element::C, ..Default::default() },
            Atom { element: Element::C, ..Default::default() }
        ],
        bonds: vec![
            vec![ Bond { tid: 1, style: None } ],
            vec![ Bond { tid: 0, style: None } ]
        ]
    });
}
```

# Versions

Purr is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Purr is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.