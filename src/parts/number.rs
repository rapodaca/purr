use std::convert::{ TryFrom, TryInto, From };
use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Number {
    value: u16
}

impl TryFrom<u16> for Number {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 1000 {
            return Ok(Number { value })
        } else {
            Err(())
        }
    }
}

impl From<&Number> for u16 {
    fn from(value: &Number) -> u16 {
        value.value
    }
}

impl TryInto<Number> for String {
    type Error = ();

    fn try_into(self) -> Result<Number, Self::Error> {
        match self.parse::<u16>() {
            Ok(number) => Ok(Number { value: number}),
            Err(_) => Err(())
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}