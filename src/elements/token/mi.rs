use super::{TokenElement, PresentationElement};
use super::super::{Element, ConcreteElement};
use ::props::{MathVariant, Directionality, Color};
use ::platform::Context;
use ::draw::{Drawable, Wrapper, Text};

pub struct MiElement {
    token_element: TokenElement
}

impl MiElement {
    pub fn new(text: String, math_variant: MathVariant, math_size: f32, dir: Directionality,
               math_color: Color, math_background: Color) -> MiElement {
        MiElement {
            token_element: TokenElement::new(
                text,
                math_variant,
                math_size,
                dir,
                math_color,
                math_background,
            )
        }
    }
}

impl Element for MiElement {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteElement::layout(self, context))
    }
}

impl<'a> ConcreteElement<'a, Wrapper<'a, PresentationElement, Text<'a, TokenElement>>> for MiElement {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationElement, Text<'a, TokenElement>> {
        <TokenElement as ConcreteElement<'a, Wrapper<'a, PresentationElement, Text<'a, TokenElement>>>>
        ::layout(&self.token_element, context)
    }
}
