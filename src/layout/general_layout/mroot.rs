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
use std::f32;

use super::super::{Layout, ElementGroup, ConcreteLayout, PresentationLayout};
use ::platform::Context;
use ::draw::{Drawable, AbsoluteLayout, AbsoluteLayoutParams, Wrapper, MeasureMode, Symbol, Line, LineParam};
use ::props::{Directionality, Color, MathSize, MathVariant, Length};
use ::paint::Point;

pub struct MrootLayout {
    pub(crate) child: Box<Layout>,
    pub(crate) degree: Box<Layout>,

    pub(crate) dir: Directionality,
    pub(crate) base_size: f32,

    pub(crate) presentation_element: PresentationLayout,
}

impl Layout for MrootLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteLayout::layout(self, context))
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, AbsoluteLayout<'a>>> for MrootLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, AbsoluteLayout<'a>> {
        let mut ll = AbsoluteLayout::new();
        ll.should_calculate_child_bounds(false);

        let ruler = context.platform().get_math_ruler(self, self.base_size);

        let vertical_gap = if self.presentation_element.display_style {
            ruler.radical_display_style_vertical_gap()
        } else {
            ruler.radical_vertical_gap()
        };

        let degree_bottom_raise_pc = ruler.radical_degree_bottom_raise_percent();
        let extra_ascender = ruler.radical_extra_ascender();
        let rule_thickness = ruler.radical_rule_thickness();
        let kern_before_degree = ruler.radical_kern_before_degree();
        let kern_after_degree = ruler.radical_kern_after_degree();

        let degree_layout = self.degree.layout(context);
        let child_layout = self.child.layout(context);

        let root_height = child_layout.bounding_box().height() + vertical_gap;

        let mut surd = Symbol::new(
            self,
            |_| "√",
            |_| &MathVariant::Normal,
            |_| false,
            |e| e.base_size,
            |_| f32::INFINITY,
            |e| e.base_size,
            |e| &e.dir,
            |e| &e.presentation_element.math_color
        );
        surd.calculate(context, &MeasureMode::Wrap, &MeasureMode::UpTo(root_height));

        let root_height = surd.bounding_box().height().max(root_height);

        let degree_pos_x = kern_before_degree;
        let degree_pos_y = ((root_height*degree_bottom_raise_pc)-degree_layout.bounding_box().height())
            .max(vertical_gap);

        let surd_pos_x = degree_pos_x + degree_layout.bounding_box().width() + kern_after_degree;
        let surd_pos_y = degree_pos_y +
            degree_layout.bounding_box().height() +
            (root_height * degree_bottom_raise_pc) -
            root_height;

        let rule_pos_x = surd_pos_x + surd.bounding_box().width();
        let rule_pos_y = surd_pos_y;

        let child_pos_x = rule_pos_x;
        let child_pos_y = rule_pos_y + rule_thickness + extra_ascender;

        let mut rule_line = Line::new(
            LineParam::Horizontal { y: 0f32 },
            self,
            rule_thickness,
            |e| &e.presentation_element.math_color
        );

        rule_line.calculate(
            context,
            &MeasureMode::UpTo(child_layout.bounding_box().width()),
            &MeasureMode::Wrap
        );

        let height = surd_pos_y+surd.bounding_box().height();
        ll.set_baseline(Some(height-(child_pos_y+child_layout.bounding_box().baseline_pos())));
        ll.set_axis(Some(height-(child_pos_y+child_layout.bounding_box().axis_pos())));

        ll.add_child(degree_layout, AbsoluteLayoutParams::new(Point::new(degree_pos_x, degree_pos_y)));
        ll.add_child(Box::new(surd), AbsoluteLayoutParams::new(Point::new(surd_pos_x, surd_pos_y)));
        ll.add_child(Box::new(rule_line), AbsoluteLayoutParams::new(Point::new(rule_pos_x, rule_pos_y)));
        ll.add_child(child_layout, AbsoluteLayoutParams::new(Point::new(child_pos_x, child_pos_y)));

        let mut wrapper = self.presentation_element.layout(context);
        wrapper.wrap(ll);
        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        wrapper
    }
}

impl MrootLayout {
}