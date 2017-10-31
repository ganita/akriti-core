use super::super::{Element, ElementGroup, ConcreteElement, PresentationElement};
use ::platform::Context;
use ::draw::{Drawable, LinearLayout, Gravity, Align, LinearLayoutParams, Wrapper, MeasureMode};
use ::props::{Directionality, Color};

pub struct MrowElement {
    elements: Vec<Box<Element>>,
    dir: Directionality,

    presentation_element: PresentationElement,
}

impl Element for MrowElement {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteElement::layout(self, context))
    }
}

impl ElementGroup for MrowElement {
    fn children(&self) -> &[Box<Element>] {
        &self.elements[..]
    }
}

impl<'a> ConcreteElement<'a, Wrapper<'a, PresentationElement, LinearLayout<'a>>> for MrowElement {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationElement, LinearLayout<'a>> {
        let mut layout: LinearLayout<'a> = LinearLayout::new();
        layout.gravity = Gravity::Horizontal;
        layout.layout_align = Align::Baseline;

        match self.dir {
            Directionality::LTR => for element in self.elements.iter() {
                layout.add_child(element.layout(context), LinearLayoutParams::new());
            },
            Directionality::RTL => for element in self.elements.iter().rev() {
                layout.add_child(element.layout(context), LinearLayoutParams::new());
            }
        }

        let mut wrapper = self.presentation_element.layout(context);
        wrapper.wrap(layout);
        wrapper.calculate(context, -1., &MeasureMode::Wrap, -1., &MeasureMode::Wrap);

        wrapper
    }
}

impl MrowElement {
    pub fn new(dir: Directionality, math_background: Color) -> MrowElement {
        MrowElement {
            elements: Vec::new(),
            dir,
            presentation_element: PresentationElement::new(math_background),
        }
    }

    pub fn add_element(&mut self, element: Box<Element>) -> &mut MrowElement {
        self.elements.push(element);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::{MiElement};
    use ::props::{MathVariant};

    #[test]
    fn mrow_works() {
        let mut mrow = MrowElement::new(Directionality::LTR, Color::transparent());
        mrow.add_element(
            Box::new(MiElement::new(
                String::from("Hello"),
                MathVariant::Normal,
                64.,
                Directionality::LTR,
                Color::RGB(0, 0, 0),
                Color::transparent()
            )
        ));


    }
}