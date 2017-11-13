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


use super::{Drawable, BoundingBox, MeasureMode};
use ::paint::{Point, Canvas};
use ::platform::Context;

pub struct Empty {
    bounding_box: BoundingBox,
}

impl Empty {
    pub fn new() -> Empty {
        Empty { bounding_box: BoundingBox::default() }
    }
}

impl Drawable for Empty {
    fn draw(&self, _: &Canvas, _: &Point) {
        // do nothing
    }

    fn calculate(&mut self, _: &Context, _: &MeasureMode, _: &MeasureMode) {
        // do nothing
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}