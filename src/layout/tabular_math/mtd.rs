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

use ::draw::{Drawable, BoundingBox, MeasureMode};
use ::props::{HAlign, VAlign, GroupAlign};
use ::paint::{Point, Canvas};
use ::platform::Context;

pub struct MtdLayout {

}


pub(in super::super::tabular_math) struct MtdDrawable<'a> {
    pub(in super::super::tabular_math) row_span: u32,
    pub(in super::super::tabular_math) col_span: u32,
    pub(in super::super::tabular_math) row_align: VAlign,
    pub(in super::super::tabular_math) col_align: HAlign,
    pub(in super::super::tabular_math) group_align: GroupAlign,

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
        unimplemented!()
    }
}