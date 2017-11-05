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


mod wrapper;                pub use self::wrapper::*;
mod text;                   pub use self::text::*;
mod absolute_layout;        pub use self::absolute_layout::*;
mod linear_layout;          pub use self::linear_layout::*;
mod fixed;                  pub use self::fixed::*;
mod padding_box;            pub use self::padding_box::*;
mod symbol;                 pub use self::symbol::*;
mod glyph;                  pub use self::glyph::*;

use ::paint::{Canvas, Point, Rect};
use ::platform::Context;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    rect: Rect,
    baseline: f32,
    axis: f32
}

impl BoundingBox {
    pub fn new(rect: Rect, baseline: f32, axis: f32) -> BoundingBox {
        BoundingBox { rect, baseline, axis }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn baseline(&self) -> f32 {
        self.baseline
    }

    pub fn axis(&self) -> f32 {
        self.axis
    }

    pub fn width(&self) -> f32 {
        self.rect.width()
    }

    pub fn height(&self) -> f32 {
        self.rect.height()
    }

    pub fn ascent(&self) -> f32 { self.height()-self.baseline }

    pub fn descent(&self) -> f32 { -self.baseline }

    pub fn baseline_pos(&self) -> f32 {
        self.height()-self.baseline()
    }

    pub fn axis_pos(&self) -> f32 {
        self.height()-self.axis()
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            rect: Rect::new(0., 0.),
            baseline: 0.0,
            axis: 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MeasureMode {
    UpTo(f32),
    Wrap
}

pub trait Drawable {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point);
    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode);

    fn bounding_box(&self) -> &BoundingBox;
}