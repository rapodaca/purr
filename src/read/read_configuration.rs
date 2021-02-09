use crate::parts::Configuration;
use super::{
    scanner::Scanner,
    Error,
    missing_character::missing_character
};

pub fn read_configuration(
    scanner: &mut Scanner
) -> Result<Option<Configuration>, Error> {
    Ok(Some(match scanner.peek() {
        Some('@') => {
            scanner.pop();

            match scanner.peek() {
                Some('@') => {
                    scanner.pop();
    
                    Configuration::TH2
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
                _ => Configuration::TH1
            }
        },
        _ => return Ok(None)
    }))
}

fn tetrahedral(scanner: &mut Scanner) -> Result<Configuration, Error> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Configuration::TH1
        },
        Some('2') => {
            scanner.pop();

            Configuration::TH2
        },
        _ => return Err(missing_character(scanner))
    })
}

fn allene(scanner: &mut Scanner) -> Result<Configuration, Error> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Configuration::AL1
        },
        Some('2') => {
            scanner.pop();

            Configuration::AL2
        },
        _ => return Err(missing_character(scanner))
    })
}

fn square_planar(scanner: &mut Scanner) -> Result<Configuration, Error> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Configuration::SP1
        },
        Some('2') => {
            scanner.pop();

            Configuration::SP2
        },
        Some('3') => {
            scanner.pop();

            Configuration::SP3
        }
        _ => return Err(missing_character(scanner))
    })
}

fn trigonal_bipyramidal(scanner: &mut Scanner) -> Result<Configuration, Error> {
    Ok(match scanner.pop() {
        Some('1') => match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => Configuration::TB10,
                Some('1') => Configuration::TB11,
                Some('2') => Configuration::TB12,
                Some('3') => Configuration::TB13,
                Some('4') => Configuration::TB14,
                Some('5') => Configuration::TB15,
                Some('6') => Configuration::TB16,
                Some('7') => Configuration::TB17,
                Some('8') => Configuration::TB18,
                Some('9') => Configuration::TB19,
                _ => unreachable!("TB1X")
            },
            _ => Configuration::TB1
        },
        Some('2') => match scanner.peek() {
            Some('0') => {
                scanner.pop();

                Configuration::TB20
            },
            _ => Configuration::TB2
        },
        Some('3') => Configuration::TB3,
        Some('4') => Configuration::TB4,
        Some('5') => Configuration::TB5,
        Some('6') => Configuration::TB6,
        Some('7') => Configuration::TB7,
        Some('8') => Configuration::TB8,
        Some('9') => Configuration::TB9,
        _ => return Err(missing_character(scanner))
    })
}

fn octahedral(scanner: &mut Scanner) -> Result<Configuration, Error> {
    Ok(match scanner.pop() {
        Some('1') => match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => Configuration::OH10,
                Some('1') => Configuration::OH11,
                Some('2') => Configuration::OH12,
                Some('3') => Configuration::OH13,
                Some('4') => Configuration::OH14,
                Some('5') => Configuration::OH15,
                Some('6') => Configuration::OH16,
                Some('7') => Configuration::OH17,
                Some('8') => Configuration::OH18,
                Some('9') => Configuration::OH19,
                _ => unreachable!("OH1X")
            },
            _ => Configuration::OH1
        },
        Some('2') => match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => Configuration::OH20,
                Some('1') => Configuration::OH21,
                Some('2') => Configuration::OH22,
                Some('3') => Configuration::OH23,
                Some('4') => Configuration::OH24,
                Some('5') => Configuration::OH25,
                Some('6') => Configuration::OH26,
                Some('7') => Configuration::OH27,
                Some('8') => Configuration::OH28,
                Some('9') => Configuration::OH29,
                _ => unreachable!("OH2X")
            },
            _ => Configuration::OH2
        },
        Some('3') => match scanner.peek() {
            Some('0') => {
                scanner.pop();

                Configuration::OH30
            },
            _ => Configuration::OH3
        },
        Some('4') => Configuration::OH4,
        Some('5') => Configuration::OH5,
        Some('6') => Configuration::OH6,
        Some('7') => Configuration::OH7,
        Some('8') => Configuration::OH8,
        Some('9') => Configuration::OH9,
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

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH1))
        )
    }

    #[test]
    fn clockwise() {
        let mut scanner = Scanner::new("@@");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH2))
        )
    }

    #[test]
    fn th_1() {
        let mut scanner = Scanner::new("@TH1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH1))
        )
    }

    #[test]
    fn th_2() {
        let mut scanner = Scanner::new("@TH2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH2))
        )
    }

    #[test]
    fn al_1() {
        let mut scanner = Scanner::new("@AL1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::AL1))
        )
    }

    #[test]
    fn tb_1() {
        let mut scanner = Scanner::new("@TB1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB1))
        )
    }

    #[test]
    fn tb_2() {
        let mut scanner = Scanner::new("@TB2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB2))
        )
    }

    #[test]
    fn tb_5() {
        let mut scanner = Scanner::new("@TB5");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB5))
        )
    }

    #[test]
    fn tb_10() {
        let mut scanner = Scanner::new("@TB10");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB10))
        )
    }

    #[test]
    fn tb_20() {
        let mut scanner = Scanner::new("@TB20");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB20))
        )
    }

    #[test]
    fn oh_1() {
        let mut scanner = Scanner::new("@OH1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH1))
        )
    }

    #[test]
    fn oh_2() {
        let mut scanner = Scanner::new("@OH2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH2))
        )
    }

    #[test]
    fn oh_3() {
        let mut scanner = Scanner::new("@OH3");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH3))
        )
    }

    #[test]
    fn oh_5() {
        let mut scanner = Scanner::new("@OH5");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH5))
        )
    }

    #[test]
    fn oh_10() {
        let mut scanner = Scanner::new("@OH10");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH10))
        )
    }

    #[test]
    fn oh_15() {
        let mut scanner = Scanner::new("@OH15");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH15))
        )
    }

    #[test]
    fn oh_20() {
        let mut scanner = Scanner::new("@OH20");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH20))
        )
    }

    #[test]
    fn oh_25() {
        let mut scanner = Scanner::new("@OH25");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH25))
        )
    }

    #[test]
    fn oh_30() {
        let mut scanner = Scanner::new("@OH30");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH30))
        )
    }

    #[test]
    fn sp_1() {
        let mut scanner = Scanner::new("@SP1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::SP1))
        )
    }

    #[test]
    fn sp_3() {
        let mut scanner = Scanner::new("@SP3");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::SP3))
        )
    }
}