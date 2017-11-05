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

pub struct Fixed {
    bounding_box: BoundingBox,
    pub background: Color,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub axis: f32,
}

impl Drawable for Fixed {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        canvas.draw_rect(
            pen_pos,
            self.bounding_box.rect(),
            &self.background
        );
        canvas.draw_rect_outline(
            pen_pos,
            self.bounding_box.rect(),
            &Color::RGB(255, 255, 255),
            1.
        );
    }

    fn calculate(&mut self, _: &Context, width_measure_mode: &MeasureMode, height_measure_mode: &MeasureMode) {
        let width = if let MeasureMode::UpTo(val) = *width_measure_mode {
            val.max(self.width)
        } else {
            self.width
        };

        let height = if let MeasureMode::UpTo(val) = *height_measure_mode {
            val.max(self.height)
        } else {
            self.height
        };

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: self.baseline,
            axis: self.axis,
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl Fixed {
    pub fn new(width: f32, height: f32, baseline: f32, axis: f32) -> Fixed {
        Fixed {
            bounding_box: BoundingBox::default(),
            background: Color::RGB(0, 0, 0),
            width,
            height,
            baseline,
            axis,
        }
    }
}