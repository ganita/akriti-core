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
use ::props::Color;
use ::paint::{Canvas, Point, Rect};
use ::platform::Context;

pub type MathBackgroundReader<T> = fn (&T) -> &Color;

pub struct Wrapper<'a, T: 'a, U: Drawable> {
    wrapped: Option<U>,
    props: &'a T,
    math_background_reader: MathBackgroundReader<T>,

    bounding_box: BoundingBox
}

impl<'a, T, U: Drawable> Drawable for Wrapper<'a, T, U> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        canvas.draw_rect(
            pen_pos,
            self.bounding_box.rect(),
            (self.math_background_reader)(self.props)
        );
        if let Some(ref wrapped) = self.wrapped {
            wrapped.draw(canvas, pen_pos);
        }
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        if let Some(val) = self.wrapped.as_mut() {
            val.calculate(context, width_mode, height_mode);
        }
        self.bounding_box = BoundingBox {
            rect: if let Some(ref val) = self.wrapped {
                val.bounding_box().rect().clone()
            } else {
                Rect::new(0., 0.)
            },
            baseline: if let Some(ref val) = self.wrapped {
                val.bounding_box().baseline()
            } else {
                0.
            },
            axis: if let Some(ref val) = self.wrapped {
                val.bounding_box().axis()
            } else {
                0.
            },
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a, T, U: Drawable> Wrapper<'a, T, U> {
    pub fn new(props: &'a T, math_background_reader: MathBackgroundReader<T>)
               -> Wrapper<'a, T, U> {
        Wrapper {
            wrapped: None,
            props,
            math_background_reader,
            bounding_box: BoundingBox::default()
        }
    }

    pub fn wrap(&mut self, drawable: U) {
        self.wrapped = Some(drawable);
    }

    pub fn get_wrapped(&self) -> Option<&U> {
        self.wrapped.as_ref()
    }

}