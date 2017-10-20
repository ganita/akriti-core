use ::props::{
    Color, 
    MathSize, 
    Directionality
};
use super::{
    Rect, 
    Point,
};

pub trait Canvas {
    fn draw_text(&self, top_left: &Point, text: &String, color: &Color, math_size: &MathSize, dir: &Directionality);
    fn draw_glyph(&self, top_left: &Point, glyph_index: u32, color: &Color, math_size: &MathSize, dir: &Directionality);
    fn draw_rect(&self, top_left: &Point, rect: &Rect, color: &Color);
    fn draw_rect_outline(&self, top_left: &Point, rect: &Rect, color: &Color, stroke_width: f32);
    fn draw_line(&self, start: &Point, end: &Point, color: &Color, stroke_width: f32);
}