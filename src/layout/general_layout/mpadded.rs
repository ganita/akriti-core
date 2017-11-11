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
use ::props::{PseudoLength, PropertyModifier, MathSize};
use ::platform::Context;
use ::draw::{Drawable, BoundingBox, MeasureMode};
use ::paint::{Point, Rect, Canvas};

pub struct MpaddedLayout {
    pub(crate) width: PropertyModifier<PseudoLength>,
    pub(crate) height: PropertyModifier<PseudoLength>,
    pub(crate) depth: PropertyModifier<PseudoLength>,

    pub(crate) lspace: PropertyModifier<PseudoLength>,
    pub(crate) voffset: PropertyModifier<PseudoLength>,

    pub(crate) child_layout: Box<Layout>,
    pub(crate) presentation_layout: PresentationLayout,
}


impl Layout for MpaddedLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        let child_drawable = self.child_layout.layout(context);
        let mut wrapper = self.presentation_layout.layout(context);

        let child_width = child_drawable.bounding_box().width();
        let child_height = child_drawable.bounding_box().height() - child_drawable.bounding_box().baseline();
        let child_depth = child_drawable.bounding_box().baseline();
        let child_axis = child_drawable.bounding_box().axis();

        let font_size = self.presentation_layout.script_level
            .get_font_size(context, &MathSize::NORMAL);

        let layout_width = self.width.value(PseudoLength::DU(child_width))
            .get_length_du(context, font_size, child_width, child_height, child_depth)
            .max(0f32);
        let layout_height = self.height.value(PseudoLength::DU(child_height))
            .get_length_du(context, font_size, child_width, child_height, child_depth)
            .max(0f32);
        let layout_depth = self.depth.value(PseudoLength::DU(child_depth))
            .get_length_du(context, font_size, child_width, child_height, child_depth)
            .max(0f32);
        let layout_axis = child_drawable.bounding_box().axis() +
            (layout_depth-child_drawable.bounding_box().baseline());

        let layout_lspace = self.lspace.value(PseudoLength::DU(0f32))
            .get_length_du(context, font_size, child_width, child_height, child_depth);
        let layout_voffset = self.voffset.value(PseudoLength::DU(0f32))
            .get_length_du(context, font_size, child_width, child_height, child_depth);

        let child_pos_x = layout_lspace;
        let child_pos_y = 0f32;

        wrapper.wrap(MpaddedDrawable {
            content: child_drawable,
            lspace: layout_lspace,
            voffset: layout_voffset,
            bounding_box: BoundingBox::new(
                Rect::new(layout_width, layout_height+layout_depth),
                layout_depth,
                layout_axis,
            ),
        });

        Box::new(wrapper)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

struct MpaddedDrawable<'a> {
    content: Box<Drawable + 'a>,

    lspace: f32,
    voffset: f32,

    bounding_box: BoundingBox
}

impl<'a> Drawable for MpaddedDrawable<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        self.content.draw(canvas,&(pen_pos+&Point::new(self.lspace, self.voffset)));
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        // do nothing
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}