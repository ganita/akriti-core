/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


use super::super::{Layout, ElementGroup, ConcreteLayout, PresentationLayout};
use ::platform::Context;
use ::draw::{Drawable, LinearLayout, Gravity, Align, LinearLayoutParams, Wrapper, MeasureMode};
use ::props::{Directionality, Color};

pub struct MrowLayout {
    elements: Vec<Box<Layout>>,
    dir: Directionality,

    presentation_element: PresentationLayout,
}

impl Layout for MrowLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteLayout::layout(self, context))
    }
}

impl ElementGroup for MrowLayout {
    fn children(&self) -> &[Box<Layout>] {
        &self.elements[..]
    }
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, LinearLayout<'a>>> for MrowLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, LinearLayout<'a>> {
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
        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        wrapper
    }
}

impl MrowLayout {
    pub fn new(dir: Directionality, math_color: Color, math_background: Color) -> MrowLayout {
        MrowLayout {
            elements: Vec::new(),
            dir,
            presentation_element: PresentationLayout::new(math_color, math_background),
        }
    }

    pub fn add_element(&mut self, element: Box<Layout>) -> &mut MrowLayout {
        self.elements.push(element);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::{MiLayout};
    use ::props::{MathVariant};

    #[test]
    fn mrow_works() {
        let mut mrow = MrowLayout::new(Directionality::LTR,
                                       Color::RGB(0, 0, 0),
                                       Color::transparent());
        mrow.add_element(
            Box::new(MiLayout::new(
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