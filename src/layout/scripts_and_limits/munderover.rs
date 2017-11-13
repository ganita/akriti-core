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

use super::super::{Layout, PresentationLayout, ConcreteLayout};
use ::props::{HAlign, MathSize};
use ::platform::Context;
use ::draw::{Drawable, BoundingBox, MeasureMode};
use ::paint::{Canvas, Point, Rect};

pub struct MunderoverLayout {
    pub(crate) accent_over: bool,
    pub(crate) accent_under: bool,
    pub(crate) align: HAlign,

    pub(crate) base: Box<Layout>,
    pub(crate) underscript: Box<Layout>,
    pub(crate) overscript: Box<Layout>,

    pub(crate) presentation_layout: PresentationLayout,
}

impl Layout for MunderoverLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        let mut wrapper = self.presentation_layout.layout(context);

        wrapper.wrap(MunderoverDrawable {
            accent_over: self.accent_over,
            accent_under: self.accent_under,
            align: self.align.clone(),
            base_size: self.presentation_layout.script_level.get_font_size(context, &MathSize::NORMAL),
            base: self.base.layout(context),
            underscript: self.underscript.layout(context),
            overscript: self.overscript.layout(context),
            bounding_box: BoundingBox::default(),
            base_pos: Point::new(0f32, 0f32),
            underscript_pos: Point::new(0f32, 0f32),
            overscript_pos: Point::new(0f32, 0f32),
        });

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        Box::new(wrapper)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}


struct MunderoverDrawable<'a> {
    accent_over: bool,
    accent_under: bool,
    align: HAlign,
    base_size: f32,

    base: Box<Drawable + 'a>,
    underscript: Box<Drawable + 'a>,
    overscript: Box<Drawable + 'a>,

    bounding_box: BoundingBox,
    base_pos: Point,
    underscript_pos: Point,
    overscript_pos: Point,
}

impl<'a> MunderoverDrawable<'a> {
    fn get_x_pos_aligned(&self, layout_width: f32, child_width: f32) -> f32 {
        match self.align {
            HAlign::Left => 0f32,
            HAlign::Center => (layout_width-child_width)/2f32,
            HAlign::Right => (layout_width-child_width),
        }
    }
}

impl<'a> Drawable for MunderoverDrawable<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        self.overscript.draw(canvas, &(pen_pos+&self.overscript_pos));
        self.base.draw(canvas, &(pen_pos+&self.base_pos));
        self.underscript.draw(canvas, &(pen_pos+&self.underscript_pos));
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        let ruler = context.platform().get_math_ruler(self.base_size);

        self.base.calculate(context, width_mode, height_mode);

        let base = &self.base;
        let overscript = &self.overscript;
        let underscript = &self.underscript;

        let has_overscript = overscript.bounding_box().width() > 0f32 || overscript.bounding_box().height() > 0f32;
        let has_underscript = underscript.bounding_box().width() > 0f32 || underscript.bounding_box().height() > 0f32;

        // Fix y position of overscript
        let overscript_y_pos = 0f32;

        let base_y_pos = if has_overscript {
            // Minimum shift between top of base and baseline of overscript
            let upper_limit_baseline_rise_min = ruler.upper_limit_baseline_rise_min();

            let mut base_y_pos = overscript_y_pos +
                overscript.bounding_box().baseline_pos() +
                upper_limit_baseline_rise_min;

            // Minimum shift between top of base and bottom of overscript
            let upper_limit_gap_min = ruler.upper_limit_gap_min();

            base_y_pos = base_y_pos.max(
                overscript_y_pos + overscript.bounding_box().height() + upper_limit_gap_min);

            base_y_pos
        } else {
            0f32
        };

        let underscript_y_pos = if has_underscript {
            // Minimum shift between bottom of base and baseline of underscript
            let lower_limit_baseline_drop_min = ruler.lower_limit_baseline_drop_min();

            let mut underscript_y_pos = base_y_pos +
                base.bounding_box().height() +
                lower_limit_baseline_drop_min -
                underscript.bounding_box().baseline_pos();

            // Minimum shift between bottom of base and top of underscript
            let lower_limit_gap_min = ruler.lower_limit_gap_min();

            underscript_y_pos = underscript_y_pos.max(
                base_y_pos + base.bounding_box().height() + lower_limit_gap_min);

            underscript_y_pos
        } else {
            base_y_pos+base.bounding_box().height()
        };

        let layout_width = base.bounding_box().width()
            .max(overscript.bounding_box().width())
            .max(underscript.bounding_box().width());

        let overscript_x_pos = self.get_x_pos_aligned(layout_width, overscript.bounding_box().width());
        let base_x_pos = self.get_x_pos_aligned(layout_width, base.bounding_box().width());
        let underscript_x_pos = self.get_x_pos_aligned(layout_width, underscript.bounding_box().width());

        self.overscript_pos = Point::new(overscript_x_pos, overscript_y_pos);
        self.base_pos = Point::new(base_x_pos, base_y_pos);
        self.underscript_pos = Point::new(underscript_x_pos, underscript_y_pos);

        let layout_height = underscript_y_pos + underscript.bounding_box().height();

        self.bounding_box = BoundingBox::new(
            Rect::new(layout_width, layout_height),
            layout_height-(base_y_pos+base.bounding_box().baseline_pos()),
            layout_height-(base_y_pos+base.bounding_box().axis_pos()),
        );
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}