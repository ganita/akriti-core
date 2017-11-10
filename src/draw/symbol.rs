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

use super::{Drawable, MeasureMode, BoundingBox, AbsoluteLayout, AbsoluteLayoutParams, Glyph, GlyphIndex, Text};
use ::platform::Context;
use ::paint::{Point, Canvas, GlyphConstructionDirection, MathRuler, GlyphAssembly, GlyphAssemblyPart};
use ::props::{Color, Directionality, MathVariant};
use ::layout::Layout;

type SymbolReader<T> = fn(&T) -> &str;
type SymmetricReader<T> = fn(&T) -> bool;
type SizeReader<T> = fn(&T) -> f32;
type DirReader<T> = fn(&T) -> &Directionality;
type ColorReader<T> = fn(&T) -> &Color;
type VariantReader<T> = fn(&T) -> &MathVariant;

pub struct Symbol<'a, T: Layout + 'a> {
    props: &'a T,
    symbol_reader: SymbolReader<T>,
    math_variant_reader: VariantReader<T>,
    symmetric_reader: SymmetricReader<T>,
    base_size_reader: SizeReader<T>,
    max_size_reader: SizeReader<T>,
    min_size_reader: SizeReader<T>,
    dir_reader: DirReader<T>,
    color_reader: ColorReader<T>,

    bounding_box: BoundingBox,
    layout: AbsoluteLayout<'a>,
}

impl<'a, T: Layout + 'a> Drawable for Symbol<'a, T> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        self.layout.draw(canvas, pen_pos);
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        let base_size = (self.base_size_reader)(self.props);
        let symbol = (self.symbol_reader)(self.props);
        let ruler = context.platform().get_math_ruler(self.props, base_size);

        let chars: Vec<char> = symbol.chars().collect();

        if chars.len() > 1 {
            self.set_text(context);
            return;
        }

        let symbol = chars[0] as u32;

        if let MeasureMode::UpTo(height) = *height_mode {
            let stretch_dir = GlyphConstructionDirection::Vertical;
            let stretched_size = base_size
                .max((self.min_size_reader)(self.props))
                .min((self.max_size_reader)(self.props))
                .max(height);

            if self.try_stretch_symbol(context,symbol, ruler, stretched_size, &stretch_dir) {
                return;
            }
        }

        if let MeasureMode::UpTo(width) = *width_mode {
            let stretch_dir = GlyphConstructionDirection::Horizontal;
            let stretched_size = base_size
                .max((self.min_size_reader)(self.props))
                .min((self.max_size_reader)(self.props))
                .max(width);

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

impl<'a, T: Layout + 'a> Symbol<'a, T> {
    pub fn new(props: &'a T, symbol_reader: SymbolReader<T>, math_variant_reader: VariantReader<T>,
               symmetric_reader: SymmetricReader<T>,
               base_size_reader: SizeReader<T>, max_size_reader: SizeReader<T>,
               min_size_reader: SizeReader<T>, dir_reader: DirReader<T>,
               color_reader: ColorReader<T>) -> Symbol<'a, T> {
        Symbol {
            props,
            symbol_reader,
            math_variant_reader,
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

        self.layout.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        self.bounding_box = self.layout.iter().next().unwrap().drawable().bounding_box().clone();
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

        let mut max_connector_overlap = f32::INFINITY;

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
        }

        let num_extenders = num_extenders as f32;
        let num_non_extenders = num_non_extenders as f32;

        // Number of times extenders needs to be repeated to get size >= stretched size
        let extender_multiplier = ((stretched_size-total_non_extender_advance-
            min_connector_overlap*(1.-num_non_extenders)) /
            (total_extender_advance-(min_connector_overlap*num_extenders))).ceil();

        // Overlap adjustment to minimize size - stretched_size
        let required_overlap = (total_non_extender_advance + (extender_multiplier * total_extender_advance)
            - stretched_size) /
            (num_non_extenders + (extender_multiplier * num_extenders) - 1.);

        // Never use overlap greater than maximum connector overlap
        let overlap = required_overlap.min(max_connector_overlap);

        #[cfg(debug_assertions)]
        {
            assert!(max_connector_overlap >= min_connector_overlap);

            let size_with_overlap_correction = extender_multiplier*total_extender_advance
                +total_non_extender_advance -
                (overlap*(num_non_extenders+(extender_multiplier*num_extenders)-1.));

            let size_without_overlap_correction = extender_multiplier*total_extender_advance
                +total_non_extender_advance -
                (min_connector_overlap*(num_non_extenders+(extender_multiplier*num_extenders)-1.));

            assert!(size_without_overlap_correction >= stretched_size);

            assert!(size_with_overlap_correction <= size_without_overlap_correction);
        }

        self.layout.clear();

        let mut pen_pos = 0f32;

        // Ordering of parts in GlyphAssembly for vertical construction is from bottom to top
        // and for horizontal construction is from left to right
        match *stretch_dir {
            GlyphConstructionDirection::Vertical => {
                for part in assembly.parts().iter().rev() {
                    pen_pos = self.add_glyph_part_to_layout(part, stretch_dir, overlap,
                                                            extender_multiplier as u32, pen_pos);
                }
            },
            GlyphConstructionDirection::Horizontal => {
                for part in assembly.parts().iter() {
                    pen_pos = self.add_glyph_part_to_layout(part, stretch_dir, overlap,
                                                            extender_multiplier as u32, pen_pos);
                }
            }
        }

        self.layout.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        self.bounding_box = self.layout.bounding_box().clone();
    }

    fn add_glyph_part_to_layout(&mut self, part: &GlyphAssemblyPart, stretch_dir: &GlyphConstructionDirection,
                                overlap: f32, multiplier: u32, pen_pos: f32) -> f32 {
        let num_iters = if part.is_extender() {
            multiplier
        } else {
            1
        };

        let mut pen_pos = pen_pos;

        for _ in 0..num_iters {
            let mut glyph = Glyph::new(
                self.props,
                GlyphIndex::Index(part.glyph_index()),
                self.base_size_reader,
                self.color_reader,
                self.dir_reader);

            if *stretch_dir == GlyphConstructionDirection::Vertical {
                glyph.set_advance(Some(part.full_advance()));
            }

            self.layout.add_child(
                Box::new(glyph),
                AbsoluteLayoutParams::new(match *stretch_dir {
                    GlyphConstructionDirection::Horizontal => Point::new(pen_pos, 0.),
                    GlyphConstructionDirection::Vertical => Point::new(0., pen_pos)
                })
            );

            pen_pos += part.full_advance()-overlap;
        }

        pen_pos
    }

    fn set_text(&mut self, context: &Context) {
        let text = Text::new(
            self.props,
            self.symbol_reader,
            self.base_size_reader,
            self.math_variant_reader,
            self.dir_reader,
            self.color_reader
        );

        self.layout.clear();
        self.layout.add_child(Box::new(text),
                              AbsoluteLayoutParams::new(Point::new(0., 0.)));

        self.layout.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        self.bounding_box = self.layout.iter().next().unwrap().drawable().bounding_box().clone();
    }

    pub fn get_layout(&self) -> &AbsoluteLayout<'a> {
        &self.layout
    }

}


#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;
    use ::test::skia::Snapshot;

    struct Test;

    impl Layout for Test {
        fn layout<'a>(&'a self, _: &Context) -> Box<Drawable + 'a> {
            unimplemented!()
        }

        fn as_any(&self) -> &Any {
            unimplemented!()
        }
    }

    #[test]
    fn test_vertical_stretching() {
        let test_element = Test {};

        let mut symbol = Symbol::new(
            &test_element,
            |_| "√",
            |_| &MathVariant::Normal,
            |_| true,
            |_| 64.,
            |_| f32::INFINITY,
            |_| 64.,
            |_| &Directionality::LTR,
            |_| &Color::RGB(0, 0, 0)
        );

        Snapshot::default().snap_drawable(&mut symbol, &MeasureMode::Wrap,
                      &MeasureMode::UpTo(1000.), "symbol_sqrt");
    }

    #[test]
    fn test_horizontal_stretching() {
        let test_element = Test {};

        let mut symbol = Symbol::new(
            &test_element,
            |_| "←",
            |_| &MathVariant::Normal,
            |_| true,
            |_| 64.,
            |_| f32::INFINITY,
            |_| 64.,
            |_| &Directionality::LTR,
            |_| &Color::RGB(0, 0, 0)
        );

        Snapshot::default().snap_drawable(&mut symbol, &MeasureMode::UpTo(1000.),
                      &MeasureMode::Wrap, "symbol_left_arrow");
    }

    #[test]
    fn test_non_stretchable() {
        let test_element = Test {};

        let mut symbol = Symbol::new(
            &test_element,
            |_| "+",
            |_| &MathVariant::Normal,
            |_| true,
            |_| 64.,
            |_| f32::INFINITY,
            |_| 64.,
            |_| &Directionality::LTR,
            |_| &Color::RGB(0, 0, 0)
        );

        Snapshot::default().snap_drawable(&mut symbol, &MeasureMode::UpTo(1000.),
                      &MeasureMode::Wrap, "symbol_plus");
    }

    #[test]
    fn test_text() {
        let test_element = Test {};

        let mut symbol = Symbol::new(
            &test_element,
            |_| "hello",
            |_| &MathVariant::Normal,
            |_| true,
            |_| 64.,
            |_| f32::INFINITY,
            |_| 64.,
            |_| &Directionality::LTR,
            |_| &Color::RGB(0, 0, 0)
        );

        Snapshot::default().snap_drawable(&mut symbol, &MeasureMode::UpTo(1000.),
                      &MeasureMode::Wrap, "symbol_text");
    }
}