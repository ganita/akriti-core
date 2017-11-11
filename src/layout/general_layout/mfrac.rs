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

use super::super::{Layout, ElementGroup, ConcreteLayout, PresentationLayout};
use ::platform::Context;
use ::draw::{Drawable, AbsoluteLayout, Wrapper, MeasureMode, Line, LineParam, AbsoluteLayoutParams, Symbol};
use ::props::{Directionality, Color, HAlign, MathSize, LineThickness, MathVariant};
use ::paint::{Point};

pub struct MfracLayout {
    pub(crate) numerator: Box<Layout>,
    pub(crate) denominator: Box<Layout>,

    pub(crate) dir: Directionality,
    pub(crate) line_thickness: LineThickness,
    pub(crate) num_align: HAlign,
    pub(crate) denom_align: HAlign,
    pub(crate) bevelled: bool,

    pub(crate) presentation_element: PresentationLayout,
}

impl Layout for MfracLayout {
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

impl ElementGroup for MfracLayout {
    fn children(&self) -> &[Box<Layout>] {
        unimplemented!()
    }
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, AbsoluteLayout<'a>>> for MfracLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, AbsoluteLayout<'a>> {
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
        line_thickness: LineThickness,
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

    pub fn layout_normal<'a>(&'a self, context: &Context) -> AbsoluteLayout<'a> {
        let mut ll = AbsoluteLayout::new();
        ll.should_calculate_child_bounds(false);

        let num_layout = self.numerator.layout(context);
        let denom_layout = self.denominator.layout(context);

        let font_size = self.presentation_element.script_level.get_font_size(
            context, &MathSize::NORMAL);
        let ruler = context.platform().get_math_ruler(self, font_size);

        let rule_thickness = self.line_thickness.get_thickness_du(
            context, font_size, ruler.fraction_rule_thickness());

        let mut line = Line::new(
            LineParam::Horizontal { y: 0. },
            self,
            rule_thickness,
            |e| &e.presentation_element.math_color
        );

        let display_style = self.presentation_element.display_style;

        let num_gap_min = if display_style {
            ruler.fraction_num_display_style_gap_min()
        } else {
            ruler.fraction_numerator_gap_min()
        };

        let num_shift_up = if display_style {
            ruler.fraction_numerator_display_style_shift_up()
        } else {
            ruler.fraction_numerator_shift_up()
        };

        let denom_gap_min = if display_style {
            ruler.fraction_denominator_display_style_gap_min()
        } else {
            ruler.fraction_denominator_gap_min()
        };

        let denom_shift_down = if display_style {
            ruler.fraction_denominator_display_style_shift_down()
        } else {
            ruler.fraction_denominator_display_style_shift_down()
        };

        let axis_height = ruler.axis_height();

        let num_baseline_shift_from_frac_axis =
            (num_shift_up-axis_height)
                .max(num_layout.bounding_box().baseline()+num_gap_min+(rule_thickness/2.0));

        let denom_baseline_shift_from_frac_axis =
            (axis_height+denom_shift_down)
                .max(denom_gap_min+denom_layout.bounding_box().baseline_pos()+(rule_thickness/2.0));

        let frac_width = num_layout.bounding_box().width().max(denom_layout.bounding_box().width());

        let num_x_pos = MfracLayout::get_aligned_x_pos(
            frac_width, num_layout.bounding_box().width(), &self.num_align);
        let num_y_pos = 0f32;

        let line_y_pos = num_y_pos+num_layout.bounding_box().baseline_pos()+
            num_baseline_shift_from_frac_axis;

        let denom_x_pos = MfracLayout::get_aligned_x_pos(
            frac_width, denom_layout.bounding_box().width(), &self.denom_align);
        let denom_y_pos = line_y_pos+
            rule_thickness+
            denom_baseline_shift_from_frac_axis-
            (denom_layout.bounding_box().baseline_pos());

        let axis = denom_y_pos-line_y_pos +
            denom_layout.bounding_box().height() -
            (rule_thickness/2f32);

        ll.set_axis(Some(axis));
        ll.set_baseline(Some(axis-axis_height));

        line.calculate(context, &MeasureMode::UpTo(frac_width), &MeasureMode::Wrap);

        ll.add_child(num_layout, AbsoluteLayoutParams::new(Point::new(num_x_pos, num_y_pos)));
        ll.add_child(Box::new(line), AbsoluteLayoutParams::new(Point::new(0., line_y_pos)));
        ll.add_child(denom_layout, AbsoluteLayoutParams::new(Point::new(denom_x_pos, denom_y_pos)));

        ll
    }

    fn get_aligned_x_pos(frac_width: f32, box_width: f32, alignment: &HAlign) -> f32 {
        match *alignment {
            HAlign::Left => 0.,
            HAlign::Center => (frac_width-box_width)/2.,
            HAlign::Right => (frac_width-box_width)
        }
    }

    pub fn layout_bevelled<'a>(&'a self, context: &Context) -> AbsoluteLayout<'a> {
        let mut ll = AbsoluteLayout::new();
        ll.should_calculate_child_bounds(false);

        let num_layout = self.numerator.layout(context);
        let denom_layout = self.denominator.layout(context);

        let font_size = self.presentation_element.script_level.get_font_size(
            context, &MathSize::NORMAL);
        let ruler = context.platform().get_math_ruler(self, font_size);

        let rule_thickness = self.line_thickness.get_thickness_du(
            context, font_size, ruler.fraction_rule_thickness());

        let horizontal_gap = ruler.skewed_fraction_horizontal_gap();
        let vertical_gap = ruler.skewed_fraction_vertical_gap();

        let num_x_pos = 0f32;
        let num_y_pos = 0f32;

        let frac_height = num_y_pos +
            num_layout.bounding_box().height() +
            vertical_gap +
            denom_layout.bounding_box().height();

        let line_p0 = Point::new(
            0f32,
            frac_height
        );

        let line_p1 = Point::new(
            horizontal_gap,
            0f32
        );

        let line_x_pos = num_x_pos + num_layout.bounding_box().width();
        let line_y_pos = 0f32;

        let denom_x_pos = line_x_pos + horizontal_gap;
        let denom_y_pos = num_y_pos + num_layout.bounding_box().height() + vertical_gap;

        let mut line = Line::new(
            LineParam::Fixed { start: line_p1, end: line_p0 },
            self,
            rule_thickness,
            |e| &e.presentation_element.math_color,
        );
        line.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        ll.add_child(num_layout, AbsoluteLayoutParams::new(Point::new(num_x_pos, num_y_pos)));
        ll.add_child(Box::new(line), AbsoluteLayoutParams::new(Point::new(line_x_pos, line_y_pos)));
        ll.add_child(denom_layout, AbsoluteLayoutParams::new(Point::new(denom_x_pos, denom_y_pos)));

        ll
    }

    pub fn set_numerator(&mut self, element: Box<Layout>) {
        self.numerator = element;
    }

    pub fn set_denominator(&mut self, element: Box<Layout>) {
        self.denominator = element;
    }
}