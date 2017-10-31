mod mi;                         pub use self::mi::*;

use super::{ConcreteElement, Element, PresentationElement};
use ::draw::{Text, Drawable, Wrapper, MeasureMode};
use ::platform::{Context};
use ::props::{MathVariant, Directionality, Color};


pub struct TokenElement {
    text: String,
    math_variant: MathVariant,
    math_size: f32,
    dir: Directionality,
    math_color: Color,

    presentation_element: PresentationElement
}

impl Element for TokenElement {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        unimplemented!()
    }
}

impl<'a> ConcreteElement<'a, Wrapper<'a, PresentationElement, Text<'a, TokenElement>>> for TokenElement {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationElement, Text<'a, TokenElement>> {
        let mut layout = self.presentation_element.layout(context);

        let text = Text::new(self, text_reader, math_size_reader,
                             math_variant_reader, dir_reader, math_color_reader);

        layout.wrap(text);
        layout.calculate(context, -1., &MeasureMode::Wrap, -1., &MeasureMode::Wrap);

        return layout;
    }
}

impl TokenElement {
    pub fn new(text: String, math_variant: MathVariant, math_size: f32, dir: Directionality,
               math_color: Color, math_background: Color) -> TokenElement {
        TokenElement {
            text,
            math_variant,
            math_size,
            dir,
            math_color,
            presentation_element: PresentationElement::new(math_background),
        }
    }
}

fn text_reader(element: &TokenElement) -> &str {
    &element.text
}

fn math_variant_reader(element: &TokenElement) -> &MathVariant {
    &element.math_variant
}

fn math_size_reader(element: &TokenElement) -> f32 {
    element.math_size
}

fn dir_reader(element: &TokenElement) -> &Directionality {
    &element.dir
}

fn math_color_reader(element: &TokenElement) -> &Color {
    &element.math_color
}