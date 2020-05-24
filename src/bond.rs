use crate::style::Style;

#[derive(PartialEq, Eq, Default, Debug)]
pub struct Bond {
    pub tid: usize,
    pub style: Option<Style>
}