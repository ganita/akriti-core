use ::props::Color;
use super::ConcreteElement;
use ::platform::Context;
use ::draw::{Drawable, Wrapper};

pub struct PresentationElement {
    pub math_background: Color
}

fn math_background_reader(element: &PresentationElement) -> &Color {
    &element.math_background
}

impl<'a, U: Drawable + 'a> ConcreteElement<'a, Wrapper<'a, PresentationElement, U>> for PresentationElement {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationElement, U> {
        Wrapper::<'a, PresentationElement, U>::new(
            self,
            math_background_reader
        )
    }
}

impl PresentationElement {
    pub fn new(math_background: Color) -> PresentationElement {
        PresentationElement { math_background }
    }
}