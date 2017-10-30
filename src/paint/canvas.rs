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