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


use ::props::{
    Color, 
    Directionality
};
use super::{
    Rect, 
    Point,
};
use ::draw::BoundingBox;

pub trait Canvas {
    fn draw_text(&self, top_left: &Point, bound: &BoundingBox, text: &str, color: &Color, size: f32, dir: &Directionality);
    fn draw_glyph(&self, top_left: &Point, bound: &BoundingBox, glyph_index: u32, color: &Color, size: f32, dir: &Directionality);
    fn draw_rect(&self, top_left: &Point, rect: &Rect, color: &Color);
    fn draw_rect_outline(&self, top_left: &Point, rect: &Rect, color: &Color, stroke_width: f32);
    fn draw_line(&self, start: &Point, end: &Point, color: &Color, stroke_width: f32);
}