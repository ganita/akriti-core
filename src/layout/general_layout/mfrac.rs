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
use ::draw::{Drawable, LinearLayout, Gravity, Align, LinearLayoutParams, Wrapper, MeasureMode,
             Line, LineParam, CrossAxisBoundMode};
use ::props::{Directionality, Color, HAlign};

pub struct MfracLayout {
    numerator: Box<Layout>,
    denominator: Box<Layout>,

    dir: Directionality,
    line_thickness: f32,
    num_align: HAlign,
    denom_align: HAlign,
    bevelled: bool,

    presentation_element: PresentationLayout,
}

impl Layout for MfracLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteLayout::layout(self, context))
    }
}

impl ElementGroup for MfracLayout {
    fn children(&self) -> &[Box<Layout>] {
        unimplemented!()
    }
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, LinearLayout<'a>>> for MfracLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, LinearLayout<'a>> {
        let layout = if self.bevelled {
            self.layout_bevelled(context)
        } else {
            self.layout_normal(context)
        };

        let mut wrapper = self.presentation_element.layout(context);
        wrapper.wrap(layout);

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        wrapper
    }
}

impl MfracLayout {
    pub fn new(
        numerator: Box<Layout>,
        denominator: Box<Layout>,
        line_thickness: f32,
        num_align: HAlign,
        denom_align: HAlign,
        bevelled: bool,
        dir: Directionality,
        math_color: Color,
        math_background: Color,
    ) -> MfracLayout {

        MfracLayout {
            numerator,
            denominator,
            line_thickness,
            num_align,
            denom_align,
            bevelled,
            dir,
            presentation_element: PresentationLayout::new(math_color, math_background),
        }
    }

    pub fn layout_normal<'a>(&'a self, context: &Context) -> LinearLayout<'a> {
        let mut ll = LinearLayout::new();
        let num_layout = self.numerator.layout(context);
        let denom_layout = self.denominator.layout(context);

        ll.gravity = Gravity::Vertical;
        ll.layout_align = Align::Center;

        let line = Line::new(
            LineParam::Horizontal { y: 0. },
            self,
            |e| e.line_thickness,
            |_| &Color::RGB(0, 0, 0)
        );

        ll.add_child(num_layout, LinearLayoutParams::new());

        ll.add_child(Box::new(line), LinearLayoutParams::new()
            .with_cross_axis_bound_mode(CrossAxisBoundMode::FillParent));

        ll.add_child(denom_layout, LinearLayoutParams::new());

        ll
    }

    pub fn layout_bevelled<'a>(&'a self, context: &Context) -> LinearLayout<'a> {
        unimplemented!()
    }

    pub fn set_numerator(&mut self, element: Box<Layout>) {
        self.numerator = element;
    }

    pub fn set_denominator(&mut self, element: Box<Layout>) {
        self.denominator = element;
    }
}