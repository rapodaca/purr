use crate::parts::{ Parity };
use super::{ Scanner, Error, missing_character };

pub fn read_parity(scanner: &mut Scanner) -> Result<Option<Parity>, Error> {
    Ok(Some(match scanner.peek() {
        Some('@') => {
            scanner.pop();

            match scanner.peek() {
                Some('@') => {
                    scanner.pop();
    
                    Parity::Clockwise
                },
                Some('A') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('L') => {
                            scanner.pop();

                            allene(scanner)?
                        },
                        _ => return Err(missing_character(scanner))
                    }
                },
                Some('O') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('H') => {
                            scanner.pop();

                            octahedral(scanner)?
                        },
                        _ => return Err(missing_character(scanner))
                    }
                },
                Some('S') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('P') => {
                            scanner.pop();

                            square_planar(scanner)?
                        },
                        _ => return Err(missing_character(scanner))
                    }
                },
                Some('T') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('B') => {
                            scanner.pop();

                            trigonal_bipyramidal(scanner)?
                        },
                        Some('H') => {
                            scanner.pop();

                            tetrahedral(scanner)?
                        },
                        _ => return Err(missing_character(scanner))
                    }
                },
                _ => Parity::Counterclockwise
            }
        },
        _ => return Ok(None)
    }))
}

fn tetrahedral(scanner: &mut Scanner) -> Result<Parity, Error> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Parity::TH1
        },
        Some('2') => {
            scanner.pop();

            Parity::TH2
        },
        _ => return Err(missing_character(scanner))
    })
}

fn allene(scanner: &mut Scanner) -> Result<Parity, Error> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Parity::AL1
        },
        Some('2') => {
            scanner.pop();

            Parity::AL2
        },
        _ => return Err(missing_character(scanner))
    })
}

fn square_planar(scanner: &mut Scanner) -> Result<Parity, Error> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Parity::SP1
        },
        Some('2') => {
            scanner.pop();

            Parity::SP2
        },
        Some('3') => {
            scanner.pop();

            Parity::SP3
        }
        _ => return Err(missing_character(scanner))
    })
}

fn trigonal_bipyramidal(scanner: &mut Scanner) -> Result<Parity, Error> {
    Ok(match scanner.pop() {
        Some('1') => match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => Parity::TB10,
                Some('1') => Parity::TB11,
                Some('2') => Parity::TB12,
                Some('3') => Parity::TB13,
                Some('4') => Parity::TB14,
                Some('5') => Parity::TB15,
                Some('6') => Parity::TB16,
                Some('7') => Parity::TB17,
                Some('8') => Parity::TB18,
                Some('9') => Parity::TB19,
                _ => unreachable!("TB1X")
            },
            _ => Parity::TB1
        },
        Some('2') => match scanner.peek() {
            Some('0') => {
                scanner.pop();

                Parity::TB20
            },
            _ => Parity::TB2
        },
        Some('3') => Parity::TB3,
        Some('4') => Parity::TB4,
        Some('5') => Parity::TB5,
        Some('6') => Parity::TB6,
        Some('7') => Parity::TB7,
        Some('8') => Parity::TB8,
        Some('9') => Parity::TB9,
        _ => return Err(missing_character(scanner))
    })
}

fn octahedral(scanner: &mut Scanner) -> Result<Parity, Error> {
    Ok(match scanner.pop() {
        Some('1') => match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => Parity::OH10,
                Some('1') => Parity::OH11,
                Some('2') => Parity::OH12,
                Some('3') => Parity::OH13,
                Some('4') => Parity::OH14,
                Some('5') => Parity::OH15,
                Some('6') => Parity::OH16,
                Some('7') => Parity::OH17,
                Some('8') => Parity::OH18,
                Some('9') => Parity::OH19,
                _ => unreachable!("OH1X")
            },
            _ => Parity::OH1
        },
        Some('2') => match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => Parity::OH20,
                Some('1') => Parity::OH21,
                Some('2') => Parity::OH22,
                Some('3') => Parity::OH23,
                Some('4') => Parity::OH24,
                Some('5') => Parity::OH25,
                Some('6') => Parity::OH26,
                Some('7') => Parity::OH27,
                Some('8') => Parity::OH28,
                Some('9') => Parity::OH29,
                _ => unreachable!("OH2X")
            },
            _ => Parity::OH2
        },
        Some('3') => match scanner.peek() {
            Some('0') => {
                scanner.pop();

                Parity::OH30
            },
            _ => Parity::OH3
        },
        Some('4') => Parity::OH4,
        Some('5') => Parity::OH5,
        Some('6') => Parity::OH6,
        Some('7') => Parity::OH7,
        Some('8') => Parity::OH8,
        Some('9') => Parity::OH9,
        _ => return Err(missing_character(scanner))
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn counterclockwise() {
        let mut scanner = Scanner::new("@");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::Counterclockwise)))
    }

    #[test]
    fn clockwise() {
        let mut scanner = Scanner::new("@@");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::Clockwise)))
    }

    #[test]
    fn th_1() {
        let mut scanner = Scanner::new("@TH1");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TH1)))
    }

    #[test]
    fn th_2() {
        let mut scanner = Scanner::new("@TH2");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TH2)))
    }

    #[test]
    fn al_1() {
        let mut scanner = Scanner::new("@AL1");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::AL1)))
    }

    #[test]
    fn tb_1() {
        let mut scanner = Scanner::new("@TB1");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TB1)))
    }

    #[test]
    fn tb_2() {
        let mut scanner = Scanner::new("@TB2");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TB2)))
    }

    #[test]
    fn tb_5() {
        let mut scanner = Scanner::new("@TB5");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TB5)))
    }

    #[test]
    fn tb_10() {
        let mut scanner = Scanner::new("@TB10");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TB10)))
    }

    #[test]
    fn tb_20() {
        let mut scanner = Scanner::new("@TB20");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::TB20)))
    }

    #[test]
    fn oh_1() {
        let mut scanner = Scanner::new("@OH1");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH1)))
    }

    #[test]
    fn oh_2() {
        let mut scanner = Scanner::new("@OH2");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH2)))
    }

    #[test]
    fn oh_3() {
        let mut scanner = Scanner::new("@OH3");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH3)))
    }

    #[test]
    fn oh_5() {
        let mut scanner = Scanner::new("@OH5");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH5)))
    }

    #[test]
    fn oh_10() {
        let mut scanner = Scanner::new("@OH10");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH10)))
    }

    #[test]
    fn oh_15() {
        let mut scanner = Scanner::new("@OH15");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH15)))
    }

    #[test]
    fn oh_20() {
        let mut scanner = Scanner::new("@OH20");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH20)))
    }

    #[test]
    fn oh_25() {
        let mut scanner = Scanner::new("@OH25");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH25)))
    }

    #[test]
    fn oh_30() {
        let mut scanner = Scanner::new("@OH30");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::OH30)))
    }

    #[test]
    fn sp_1() {
        let mut scanner = Scanner::new("@SP1");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::SP1)))
    }

    #[test]
    fn sp_3() {
        let mut scanner = Scanner::new("@SP3");

        assert_eq!(read_parity(&mut scanner), Ok(Some(Parity::SP3)))
    }
}