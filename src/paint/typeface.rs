
#[derive(Debug)]
pub struct TypeFace {
    font_family: String,
    font_style: String
}

impl TypeFace {
    pub fn new(family: String, style: String) -> TypeFace {
        TypeFace {
            font_style: style,
            font_family: family,
        }
    }
}