# Purr

Primitives for reading the SMILES language in Rust. For details, see: *[Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)*.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
purr = "0.6"
```

## Examples

Parse ethanol:

```rust
use purr::read::{ read, Error };
use purr::valence::implicit_hydrogens;
use purr::mol::{ Mol, Atom, Bond, Element };

fn main() -> Result<(), Error> {
    let atoms = read(&"OC[CH3]")?;
     assert_eq!(atoms, vec![
         Atom {
             nub: Nub { element: Element::O, ..Default::default() },
             bonds: vec![
                 Bond { tid: 1, style: None }
             ]
         },
         Atom {
             nub: Default::default(),
             bonds: vec![
                 Bond { tid: 0, style: None },
                 Bond { tid: 2, style: None }
             ]
         },
         Atom {
             nub: Nub {
                 hcount: Some(3), charge: Some(0), ..Default::default()
             },
             bonds: vec![
                Bond { tid: 1, style: None }
             ]
         }
     ]);

     assert_eq!(implicit_hydrogens(&atoms[0]), Ok(Some(1)));
     assert_eq!(implicit_hydrogens(&atoms[1]), Ok(Some(2)));
     assert_eq!(implicit_hydrogens(&atoms[2]), Ok(None));

     Ok(())
}
```

Errors are reported with the character index.

```rust
use purr::read::{ read, Error };

fn main() {
    let mol = read(&"OCCXC");

    assert_eq!(mol.err(), Some(Error::InvalidCharacter(3)));
}
```

# Versions

Purr is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Purr is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.