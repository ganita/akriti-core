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

use std::any::Any;

use super::super::{Layout, ElementGroup, ConcreteLayout, PresentationLayout, MfracLayout, MoLayout};
use ::platform::Context;
use ::draw::{Drawable, LinearLayout, Gravity, Align, LinearLayoutParams, Wrapper, MeasureMode, CrossAxisBoundMode};
use ::props::{Directionality, Color};

pub struct MrowLayout {
    pub(crate) elements: Vec<Box<Layout>>,
    pub(crate) dir: Directionality,

    pub(crate) presentation_element: PresentationLayout,
}

impl Layout for MrowLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteLayout::layout(self, context))
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
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
                layout.add_child(element.layout(context),
                                 MrowLayout::get_linear_layout_params_for_element(element.as_ref()));
            },
            Directionality::RTL => for element in self.elements.iter().rev() {
                layout.add_child(element.layout(context),
                                 MrowLayout::get_linear_layout_params_for_element(element.as_ref()));
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

    fn get_linear_layout_params_for_element(element: &Layout) -> LinearLayoutParams {
        if element.as_any().is::<MfracLayout>() {
            return LinearLayoutParams::new().with_align(Some(Align::Axis));
        }

        if let Some(mo_layout) = element.as_any().downcast_ref::<MoLayout>() {
            if mo_layout.stretchy {
                return LinearLayoutParams::new()
                    .with_cross_axis_bound_mode(CrossAxisBoundMode::FillParent)
                    .with_weight(1f32);
            }
        }

        return LinearLayoutParams::new();
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