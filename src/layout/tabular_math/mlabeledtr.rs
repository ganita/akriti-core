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

use super::super::{Layout, ConcreteLayout};
use super::{MtdDrawable};
use ::platform::Context;
use ::draw::{Drawable, BoundingBox, MeasureMode};
use ::paint::{Canvas, Point};

pub struct MlabeledtrLayout {

}

impl<'a> ConcreteLayout<'a, MlabeledtrDrawable<'a>> for MlabeledtrLayout {
    fn layout(&'a self, context: &Context) -> MlabeledtrDrawable<'a> {
        unimplemented!()
    }
}

impl Layout for MlabeledtrLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        unimplemented!()
    }

    fn as_any(&self) -> &Any {
        unimplemented!()
    }

    fn as_any_mut(&mut self) -> &mut Any {
        unimplemented!()
    }
}

pub(in super::super::tabular_math) struct MlabeledtrDrawable<'a> {
    pub(in super::super::tabular_math) column: Vec<MtdDrawable<'a>>,
}

impl<'a> Drawable for MlabeledtrDrawable<'a> {
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