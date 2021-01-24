use crate::parts::BondKind;

pub fn write_bond_kind(kind: &BondKind) -> String {
    String::from(match kind {
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

        assert_eq!(write_bond_kind(&kind), "")
    }

    #[test]
    fn single() {
        let kind = BondKind::Single;

        assert_eq!(write_bond_kind(&kind), "-")
    }
}