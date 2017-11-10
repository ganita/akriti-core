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


use std::char::from_u32;

use super::{Drawable, BoundingBox, MeasureMode};
use ::paint::{Point, Canvas};
use ::layout::Layout;
use ::platform::Context;
use ::props::{Color, Directionality};

pub type SizeReader<T> = fn(&T) -> f32;
pub type DirReader<T> = fn(&T) -> &Directionality;
pub type ColorReader<T> = fn(&T) -> &Color;

pub enum GlyphIndex {
    Char(u32),
    Index(u32),
}

pub struct Glyph<'a, T: Layout + 'a> {
    glyph_index: GlyphIndex,
    element: &'a T,
    size_reader: SizeReader<T>,
    dir_reader: DirReader<T>,
    color_reader: ColorReader<T>,

    bounding_box: BoundingBox,
}

impl<'a, T: Layout + 'a> Drawable for Glyph<'a, T> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        match self.glyph_index {
            GlyphIndex::Index(index) => {
                canvas.draw_glyph(pen_pos, &self.bounding_box, index,
                                  (self.color_reader)(self.element),
                                  (self.size_reader)(self.element),
                                  (self.dir_reader)(self.element));
            },
            GlyphIndex::Char(unicode) => {
                canvas.draw_text(pen_pos, &self.bounding_box,
                                 &from_u32(unicode).unwrap().to_string(),
                                 (self.color_reader)(self.element),
                                 (self.size_reader)(self.element),
                                 (self.dir_reader)(self.element));
            }
        }
    }

    fn calculate(&mut self, context: &Context, _: &MeasureMode, _: &MeasureMode) {
        let base_size = (self.size_reader)(self.element);
        let ruler = context.platform().get_math_ruler(self.element, base_size);
        let dir = (self.dir_reader)(self.element);

        let bounds = match self.glyph_index {
            GlyphIndex::Char(unicode) => ruler.measure_char(unicode, dir),
            GlyphIndex::Index(index) => ruler.measure_glyph(index, dir),
        };

        let factor = if bounds.height() > base_size {
            bounds.height() / base_size
        } else {
            1.
        };

        self.bounding_box = BoundingBox {
            rect: bounds,
            baseline: -ruler.descent()*factor,
            axis: (ruler.axis_height()-ruler.descent())*factor,
        };
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a, T: Layout + 'a> Glyph<'a, T> {
    pub fn new(element: &'a T, glyph_index: GlyphIndex, size_reader: SizeReader<T>,
               color_reader: ColorReader<T>, dir_reader: DirReader<T>) -> Glyph<'a, T> {
        Glyph {
            glyph_index,
            element,
            size_reader,
            dir_reader,
            color_reader,
            bounding_box: BoundingBox::default(),
        }
    }
}