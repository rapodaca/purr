# Purr

Primitives for reading the SMILES language in Rust. For details, see: *[Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)*.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
purr = "0.5"
```

## Examples

Parse ethanol:

```rust
use purr::read::{ read, Error };
use purr::valence::implicit_hydrogens;
use purr::mol::{ Mol, Atom, Bond, Element };

fn main() -> Result<(), Error> {
    let mol = read(&"OC[CH3]")?;

    assert_eq!(mol, Mol {
        atoms: vec![
            Atom { element: Element::O, ..Default::default() },
            Atom { ..Default::default() },
            Atom { hcount: Some(3), charge: Some(0), ..Default::default() }
        ],
        bonds: vec![
            vec![ Bond { tid: 1, style: None } ],
            vec![ Bond { tid: 0, style: None }, Bond { tid: 2, style: None } ],
            vec![ Bond { tid: 1, style: None } ]
        ]
    });

    assert_eq!(implicit_hydrogens(&mol.atoms[0], &mol.bonds[0]), Some(1));
    assert_eq!(implicit_hydrogens(&mol.atoms[1], &mol.bonds[1]), Some(2));
    assert_eq!(implicit_hydrogens(&mol.atoms[2], &mol.bonds[2]), None);

    Ok(())
}
```

Syntax errors are caught and the index of the error is returned. Some semantic
errors are caught, but index reporting is not currently available.

```rust
use purr::read::read;

fn main() {
    let mol = read(&"OCCXC");

    assert_eq!(mol.err(), Some(Err::InvalidCharacter(3)));
}
```

# Versions

Purr is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Purr is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.