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

/*
use ::akriti_core::paint::{Canvas, Point, Rect};
use ::akriti_core::draw::BoundingBox;
use ::cairo::{Context, Glyph};
use ::akriti_core::props::{Color, Directionality};

pub struct CairoCanvas<'a> {
    context: &'a Context,
}

impl<'a> CairoCanvas<'a> {
    pub fn new(context: &'a Context) -> CairoCanvas {
        CairoCanvas { context }
    }
}

fn set_source_color(context: &Context, color: &Color) {
    context.set_source_rgba(
        (color.r() as f64) /255.,
        (color.g() as f64) /255.,
        (color.b() as f64) /255.,
        (color.a() as f64) /255.,
    )
}

impl<'a> Canvas for CairoCanvas<'a> {
    fn draw_text(&self, top_left: &Point, bound: &BoundingBox, text: &str, color: &Color, size: f32, _: &Directionality) {
        println!("{:?}", top_left);
        self.context.move_to(top_left.x() as f64, (top_left.y()+bound.baseline_pos()) as f64);
        set_source_color(self.context, color);
        self.context.set_font_size(size as f64);
        self.context.show_text(text);
    }

    fn draw_glyph(&self, top_left: &Point, bound: &BoundingBox, glyph_index: u32, color: &Color, size: f32, _: &Directionality) {
        set_source_color(self.context, color);
        self.context.set_font_size(size as f64);
        self.context.show_glyphs(&[Glyph { index: glyph_index as u64, x: top_left.x() as f64,
            y: (top_left.y()+bound.baseline_pos()) as f64 }])
    }

    fn draw_rect(&self, top_left: &Point, rect: &Rect, color: &Color) {
        set_source_color(self.context, color);
        self.context.rectangle(top_left.x() as f64, top_left.y() as f64, rect.width() as f64,
                               rect.height() as f64);
        self.context.fill()
    }

    fn draw_rect_outline(&self, top_left: &Point, rect: &Rect, color: &Color, stroke_width: f32) {
        set_source_color(self.context, color);
        self.context.rectangle(top_left.x() as f64, top_left.y() as f64, rect.width() as f64,
                               rect.height() as f64);
        self.context.set_line_width(stroke_width as f64);
        self.context.stroke()
    }

    fn draw_line(&self, start: &Point, end: &Point, color: &Color, stroke_width: f32) {
        set_source_color(self.context, color);
        self.context.set_line_width(stroke_width as f64);
        self.context.move_to(start.x() as f64, start.y() as f64);
        self.context.line_to(end.x() as f64, end.y() as f64);
        self.context.stroke();
    }
}*/