#[derive(Debug, Clone, PartialEq)]
pub enum LineBreak {
    Auto,
    Newline,
    NoBreak,
    GoodBreak,
    BadBreak
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakStyle {
    Before,
    After,
    Duplicate,
    InfixLineBreak,
    Style
}

impl LineBreakStyle {
    pub fn from_str(val: &str) -> Result<LineBreakStyle, String> {
        match val {
            "before" => Ok(LineBreakStyle::Before),
            "after" => Ok(LineBreakStyle::After),
            "duplicate" => Ok(LineBreakStyle::Duplicate),
            "infixlinebreak-style" => Ok(LineBreakStyle::InfixLineBreak),
            _ => Err(format!("Unknown line break style {}", val))
        }
    }
}

