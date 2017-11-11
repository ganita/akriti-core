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
use ::platform::Context;
use ::paint::{Canvas, Point};

pub struct Phantom<'a> {
    wrapped: Option<Box<Drawable + 'a>>,

    bounding_box: BoundingBox,
}

impl<'a> Phantom<'a> {
    pub fn new() -> Phantom<'a> {
        Phantom { wrapped: None, bounding_box: BoundingBox::default() }
    }

    pub fn wrap(&mut self, wrapped: Option<Box<Drawable + 'a>>) {
        self.wrapped = wrapped;
    }
}

impl<'a> Drawable for Phantom<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        // do nothing
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        if let Some(ref mut wrapped) = self.wrapped {
            wrapped.calculate(context, width_mode, height_mode);
            self.bounding_box = wrapped.bounding_box().clone();
        } else {
            self.bounding_box = BoundingBox::default()
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}