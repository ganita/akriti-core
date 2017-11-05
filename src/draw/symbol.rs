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


use std::f32;

use super::{Drawable, MeasureMode, BoundingBox, AbsoluteLayout, AbsoluteLayoutParams, Glyph, GlyphIndex};
use ::platform::Context;
use ::paint::{Point, Canvas, GlyphConstructionDirection, MathRuler, GlyphAssembly};
use ::props::{Color, Directionality};
use ::elements::Element;

type SymbolReader<T> = fn(&T) -> u32;
type SymmetricReader<T> = fn(&T) -> bool;
type SizeReader<T> = fn(&T) -> f32;
type DirReader<T> = fn(&T) -> &Directionality;
type ColorReader<T> = fn(&T) -> &Color;

pub struct Symbol<'a, T: Element + 'a> {
    props: &'a T,
    symbol_reader: SymbolReader<T>,
    symmetric_reader: SymmetricReader<T>,
    base_size_reader: SizeReader<T>,
    max_size_reader: SizeReader<T>,
    min_size_reader: SizeReader<T>,
    dir_reader: DirReader<T>,
    color_reader: ColorReader<T>,

    bounding_box: BoundingBox,
    layout: AbsoluteLayout<'a>,
}

impl<'a, T: Element + 'a> Drawable for Symbol<'a, T> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        self.layout.draw(canvas, pen_pos);
    }

    fn calculate(&mut self, context: &Context, width: f32, width_mode: &MeasureMode, height: f32,
                 height_mode: &MeasureMode) {
        let base_size = (self.base_size_reader)(self.props);
        let symbol = (self.symbol_reader)(self.props);
        let ruler = context.platform().get_math_ruler(self.props, base_size);

        if *height_mode == MeasureMode::UpTo {
            let stretch_dir = GlyphConstructionDirection::Vertical;
            let stretched_size = base_size
                .max((self.min_size_reader)(self.props))
                .min((self.max_size_reader)(self.props))
                .min(height);

            if self.try_stretch_symbol(context,symbol, ruler, stretched_size, &stretch_dir) {
                return;
            }
        }

        if *width_mode == MeasureMode::UpTo {
            let stretch_dir = GlyphConstructionDirection::Horizontal;
            let stretched_size = base_size
                .max((self.min_size_reader)(self.props))
                .min((self.max_size_reader)(self.props))
                .min(width);

            if self.try_stretch_symbol(context,symbol, ruler, stretched_size, &stretch_dir) {
                return;
            }
        }

        self.set_single_glyph(context,GlyphIndex::Char(symbol));
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a, T: Element + 'a> Symbol<'a, T> {
    pub fn new(props: &'a T, symbol_reader: SymbolReader<T>, symmetric_reader: SymmetricReader<T>,
               base_size_reader: SizeReader<T>, max_size_reader: SizeReader<T>,
               min_size_reader: SizeReader<T>, dir_reader: DirReader<T>,
               color_reader: ColorReader<T>) -> Symbol<'a, T> {
        Symbol {
            props,
            symbol_reader,
            symmetric_reader,
            base_size_reader,
            max_size_reader,
            min_size_reader,
            dir_reader,
            color_reader,
            bounding_box: BoundingBox::default(),
            layout: AbsoluteLayout::new(),
        }
    }

    fn set_single_glyph(&mut self, context: &Context, glyph: GlyphIndex) {
        let glyph = Glyph::new(
            self.props,
            glyph,
            self.base_size_reader,
            self.color_reader,
            self.dir_reader
        );

        self.layout.clear();
        self.layout.add_child(Box::new(glyph),
                              AbsoluteLayoutParams::new(Point::new(0., 0.)));

        self.layout.calculate(context, -1., &MeasureMode::Wrap, -1., &MeasureMode::Wrap);

        self.bounding_box = self.layout.bounding_box().clone();
    }

    fn try_stretch_symbol(&mut self, context: &Context, symbol: u32, ruler: &MathRuler,
                          stretched_size: f32, stretch_dir: &GlyphConstructionDirection) -> bool {
        let variants = ruler.glyph_variants(symbol, stretch_dir);

        for variant in variants {
            if variant.advance() >= stretched_size {
                self.set_single_glyph(context, GlyphIndex::Index(variant.glyph_index()));
                return true;
            }
        }

        let assembly = ruler.glyph_assembly(symbol, stretch_dir);
        if assembly.parts().len() >= 2 {
            self.set_glyph_assembly(context, assembly, &stretch_dir, stretched_size, ruler);
            return true;
        }

        return false;
    }

    fn set_glyph_assembly(&mut self, context: &Context, assembly: GlyphAssembly,
                          stretch_dir: &GlyphConstructionDirection,
                          stretched_size: f32, ruler: &MathRuler) {
        let min_connector_overlap = ruler.minimum_connector_overlap(stretch_dir);

        let mut num_extenders = 0;
        let mut total_extender_advance = 0f32;

        let mut num_non_extenders = 0;
        let mut total_non_extender_advance = 0f32;

        let mut max_connector_overlap = 0f32;

        for (index, part) in assembly.parts().iter().enumerate() {
            if part.is_extender() {
                num_extenders += 1;
                total_extender_advance += part.full_advance();
            } else {
                num_non_extenders += 1;
                total_non_extender_advance += part.full_advance();
            }

            if index != 0 {
                max_connector_overlap = max_connector_overlap.min(part.start_connector_length());
            }

            max_connector_overlap = max_connector_overlap.min(part.full_advance());

            if index != assembly.parts().len()-1 {
                max_connector_overlap = max_connector_overlap.min(part.end_connector_length());
            }

            max_connector_overlap = max_connector_overlap
                .min(part.start_connector_length())
                .min(part.end_connector_length())
                .min(part.full_advance());
        }

        let num_extenders = num_extenders as f32;
        let num_non_extenders = num_non_extenders as f32;

        let extender_multiplier = ((stretched_size + (min_connector_overlap * (num_non_extenders + 1.))
            - total_non_extender_advance) /
            (total_extender_advance + (min_connector_overlap * num_extenders))).ceil();

        let required_overlap = (total_non_extender_advance + (extender_multiplier * total_extender_advance)
            - stretched_size) /
            (num_non_extenders + (extender_multiplier * num_extenders) - 1.);

        let overlap = required_overlap.min(max_connector_overlap);

        self.layout.clear();

        let mut pen_pos = 0f32;
        for part in assembly.parts().iter() {
            self.layout.add_child(
                Box::new(Glyph::new(
                    self.props,
                    GlyphIndex::Index(part.glyph_index()),
                    self.base_size_reader,
                    self.color_reader,
                    self.dir_reader)
                ),
                AbsoluteLayoutParams::new(match *stretch_dir {
                    GlyphConstructionDirection::Horizontal => Point::new(pen_pos, 0.),
                    GlyphConstructionDirection::Vertical => Point::new(0., pen_pos)
                })
            );

            pen_pos += part.full_advance()-overlap;
        }

        self.layout.calculate(context, -1., &MeasureMode::Wrap, -1., &MeasureMode::Wrap);

        self.bounding_box = self.layout.bounding_box().clone();
    }
}