use super::Length;

#[derive(Debug, Clone, PartialEq)]
pub enum IndentAlign {
    Left,
    Center,
    Right,
    Auto,
    Id
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndentAlignFirstLast {
    Left,
    Center,
    Right,
    Auto,
    Id,
    IndentAlign
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndentShiftFirstLast {
    Length(Length),
    IndentShift
}