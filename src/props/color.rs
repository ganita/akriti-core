use std::u8;
use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    RGB(u8, u8, u8),
    ARGB(u8, u8, u8, u8)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorParseError {}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Color should be of format #AARRGGBB or #RRGGBB".fmt(f)
    }
}

impl Error for ColorParseError {

    fn description(&self) -> &str {
        "Color should be of format #AARRGGBB or #RRGGBB"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Color {

    fn parse_color_byte(hex: &str) -> Result<u8, ColorParseError> {
        match u8::from_str_radix(hex, 16) {
            Ok(val) => Ok(val),
            Err(_) => Err(ColorParseError{}),
        }
    }

    pub fn parse(hex: &str) -> Result<Color, ColorParseError> {
        let hex = hex.trim().to_lowercase();
        if hex[0..1] != "#"[..] {
            return Err(ColorParseError {});
        }
        let hex = hex[1..].to_string();

        if hex.len() != 8 && hex.len() != 6 {
            return Err(ColorParseError {})
        }

        let a: Option<u8> = if hex.len() == 8 {
            Some(Color::parse_color_byte(&hex[0..2])?)
        } else {
            None
        };

        let hex = if hex.len() == 8 {
            &hex[2..]
        } else {
            &hex[..]
        };

        let r = Color::parse_color_byte(&hex[0..2])?;
        let g = Color::parse_color_byte(&hex[2..4])?;
        let b = Color::parse_color_byte(&hex[4..6])?;

        if let Some(val) = a {
            Ok(Color::ARGB(val, r, g, b))
        } else {
            Ok(Color::RGB(r, g, b))
        }
    }

    pub fn transparent() -> Color {
        Color::ARGB(255, 0, 0, 0)
    }

    pub fn black() -> Color {
        Color::RGB(0, 0, 0)
    }

    pub fn a(&self) -> u8 {
       match *self {
           Color::ARGB(a, _, _, _) => a,
           Color::RGB(_, _, _) => 255
       }
    }

    pub fn r(&self) -> u8 {
        match *self {
            Color::ARGB(_, r, _, _) | Color::RGB(r, _, _) => r,
        }
    }

    pub fn g(&self) -> u8 {
        match *self {
            Color::ARGB(_, _, g, _) | Color::RGB(_, g, _) => g,
        }
    }

    pub fn b(&self) -> u8 {
        match *self {
            Color::ARGB(_, _, _, b) | Color::RGB(_, _, b) => b,
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_hex_rrggbb() {
        assert_eq!(
            Color::parse("#A8B8C8").unwrap(),
            Color::RGB(168, 184, 200)
        );
    }

    #[test]
    fn should_parse_hex_aarrggbb() {
        assert_eq!(
            Color::parse("#38A8B8C8").unwrap(),
            Color::ARGB(56, 168, 184, 200)
        );
    }

    #[test]
    fn should_not_parse_color_not_starting_with_hash() {
        Color::parse("38A8B8C8")
            .expect_err("Should not parse color not starting with #");
    }

    #[test]
    fn should_not_parse_color_with_invalid_hex() {
        Color::parse("#Z8A8B8C8")
            .expect_err("Should not parse color not with invalid hex");
    }

    #[test]
    fn should_not_parse_color_with_invalid_length() {
        Color::parse("#A8A8B8C")
            .expect_err("Should not parse color not with invalid length");

        Color::parse("#A8A8B8C8AB")
            .expect_err("Should not parse color not with invalid length");
    }

}