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

use super::super::{Layout, ConcreteLayout, PresentationLayout};
use ::draw::{Drawable, BoundingBox, MeasureMode, Wrapper};
use ::props::{HAlign, VAlign, GroupAlign};
use ::paint::{Point, Canvas};
use ::platform::Context;

pub struct MtdLayout {
    pub(crate) row_span: u32,
    pub(crate) column_span: u32,
    pub(crate) row_align: VAlign,
    pub(crate) column_align: HAlign,
    pub(crate) group_align: Vec<GroupAlign>,
    pub(crate) content: Option<Box<Layout>>,
    pub(crate) presentation_layout: PresentationLayout,
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, MtdDrawable<'a>>> for MtdLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, MtdDrawable<'a>> {
        let mut wrapper = self.presentation_layout.layout(context);

        wrapper.wrap(MtdDrawable {
            row_span: self.row_span,
            col_span: self.column_span,
            row_align: self.row_align.clone(),
            col_align: self.column_align.clone(),
            group_align_reader: |s| &s.group_align,
            content: self.content.as_ref().and_then(|c| Some(c.layout(context))),
            bounding_box: BoundingBox::default(),
        });

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        wrapper
    }
}

impl Layout for MtdLayout {
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


type GroupAlignReader = fn(&MtdLayout) -> &Vec<GroupAlign>;

pub(in super::super::tabular_math) struct MtdDrawable<'a> {
    pub(in super::super::tabular_math) row_span: u32,
    pub(in super::super::tabular_math) col_span: u32,
    pub(in super::super::tabular_math) row_align: VAlign,
    pub(in super::super::tabular_math) col_align: HAlign,
    pub(in super::super::tabular_math) group_align_reader: GroupAlignReader,

    pub(in super::super::tabular_math) content: Option<Box<Drawable + 'a>>,

    pub(in super::super::tabular_math) bounding_box: BoundingBox,
}

impl<'a> Drawable for MtdDrawable<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        unimplemented!()
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        unimplemented!()
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}