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


use std::cell::RefCell;
use std::ops::Deref;
use std::any::Any;
use std::fs::{File};
use std::io::Error;
use std::path::Path;
use std::io::Write;

use ::paint::{Point, Rect};
use ::draw::{BoundingBox};
use ::props::{Directionality, Color};
use ::skia_sys::{Surface, Paint, ImageInfo, ColorType, AlphaType, Typeface,
             Color as SkiaColor, TextEncoding, Rect as SkiaRect, Path as SkiaPath};

pub struct Canvas {
    surface: Surface,
    paint: RefCell<Paint>
}

impl Canvas {
    pub fn new(width: f32, height: f32, typeface: &Typeface) -> Canvas {
        let surface = Surface::new_raster(&ImageInfo {
            width: width.ceil() as i32,
            height: height.ceil() as i32,
            colorType: ColorType::default_8888(),
            alphaType: AlphaType::PREMUL_SK_ALPHATYPE,
        }, &None);

        let mut paint = Paint::new();
        paint.set_typeface(typeface);

        Canvas {
            surface,
            paint: RefCell::new(paint),
        }
    }

    pub fn snapshot(&self, path: &Path) -> Result<(), Error> {
        let data = self.surface.new_image_snapshot().encode();
        let mut file = File::create(path)?;

        file.write(data.get_data())?;

        Ok(())
    }
}

impl ::paint::Canvas for Canvas {
    fn draw_text(&self, top_left: &Point, bound: &BoundingBox, text: &str, color: &Color, size: f32,
                 _: &Directionality) {

        let mut paint = self.paint.borrow_mut();
        paint.set_color(&SkiaColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        });
        paint.set_text_size(size);
        paint.set_text_encoding(TextEncoding::kUTF8_TextEncoding);
        paint.set_antialias(true);
        paint.set_stroke(false);

        self.surface.get_canvas().draw_text(text, top_left.x(), top_left.y()+bound.baseline_pos(),
                                            paint.deref());
    }

    fn draw_glyph(&self, top_left: &Point, bounds: &BoundingBox, glyph_index: u32, color: &Color,
                  size: f32, _: &Directionality) {
        let mut paint = self.paint.borrow_mut();
        paint.set_color(&SkiaColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        });
        paint.set_text_size(size);
        paint.set_text_encoding(TextEncoding::kGlyphID_TextEncoding);
        paint.set_antialias(true);
        paint.set_stroke(false);

        let baseline_pos = if bounds.rect().height()-size > 5f32 {
            bounds.rect().height()-size
        } else {
            bounds.baseline_pos()
        };

        self.surface.get_canvas().draw_blob(&[glyph_index as u16], top_left.x(),
                                            top_left.y()+baseline_pos, paint.deref());
    }

    fn draw_rect(&self, top_left: &Point, rect: &Rect, color: &Color) {
        let mut paint = self.paint.borrow_mut();
        paint.set_color(&SkiaColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        });
        paint.set_antialias(true);
        paint.set_stroke(false);

        self.surface.get_canvas().draw_rect(&SkiaRect {
            left: top_left.x(),
            top: top_left.y(),
            right: top_left.x()+rect.width(),
            bottom: top_left.y()+rect.height(),
        }, paint.deref());
    }

    fn draw_rect_outline(&self, top_left: &Point, rect: &Rect, color: &Color, stroke_width: f32) {
        let mut paint = self.paint.borrow_mut();
        paint.set_color(&SkiaColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        });
        paint.set_antialias(true);
        paint.set_stroke(true);
        paint.set_stroke_width(stroke_width);

        self.surface.get_canvas().draw_rect(&SkiaRect {
            left: top_left.x(),
            top: top_left.y(),
            right: top_left.x()+rect.width(),
            bottom: top_left.y()+rect.height(),
        }, paint.deref());
    }

    fn draw_line(&self, start: &Point, end: &Point, color: &Color, stroke_width: f32) {
        let mut paint = self.paint.borrow_mut();
        paint.set_color(&SkiaColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        });
        paint.set_antialias(true);
        paint.set_stroke(true);
        paint.set_stroke_width(stroke_width);

        let mut path = SkiaPath::new();
        path.move_to(start.x(), start.y());
        path.line_to(end.x(), end.y());

        self.surface.get_canvas().draw_path(&path, paint.deref());
    }

    fn as_any(&self) -> &Any {
        self
    }
}