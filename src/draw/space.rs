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
use ::paint::{Rect, Point, Canvas};
use ::platform::Context;
use ::props::Color;

pub struct Space {
    bounding_box: BoundingBox,

    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub axis: f32,
}

impl Drawable for Space {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        // nothing to draw
    }

    fn calculate(&mut self, _: &Context, _: &MeasureMode, _: &MeasureMode) {
        self.bounding_box = BoundingBox {
            rect: Rect::new(self.width, self.height),
            baseline: self.baseline,
            axis: self.axis,
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl Space {
    pub fn new(width: f32, height: f32, baseline: f32, axis: f32) -> Space {
        Space {
            bounding_box: BoundingBox::default(),
            
            width,
            height,
            baseline,
            axis,
        }
    }
    
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }
    
    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn set_baseline(&mut self, baseline: f32) {
        self.baseline = baseline;
    }

    pub fn get_baseline(&self) -> f32 {
        self.baseline
    }

    pub fn set_axis(&mut self, axis: f32) {
        self.axis = axis;
    }

    pub fn get_axis(&self) -> f32 {
        self.axis
    }
}