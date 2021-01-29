use crate::parts::BondKind;

pub fn write_bond_kind(kind: &BondKind, out: &mut String) {
    out.push_str(match kind {
        BondKind::Elided => "",
        BondKind::Single => "-",
        BondKind::Double => "=",
        BondKind::Triple => "#",
        BondKind::Quadruple => "$",
        BondKind::Up => "/",
        BondKind::Down => "\\",
        BondKind::Aromatic => ":"
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn elided() {
        let kind = BondKind::Elided;
        let mut out = String::new();

        write_bond_kind(&kind, &mut out);

        assert_eq!(out, "")
    }

    #[test]
    fn single() {
        let kind = BondKind::Single;
        let mut out = String::new();

        write_bond_kind(&kind, &mut out);

        assert_eq!(out, "-")
    }
}