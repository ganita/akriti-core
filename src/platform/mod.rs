mod context;                pub use self::context::Context;

use ::paint::{
    TextRuler, 
    MathRuler
};
use ::elements::Element;

pub trait Platform {
    fn get_text_ruler(&self, element: &Element, size: f32) -> &TextRuler;
    fn get_math_ruler(&self, element: &Element, size: f32) -> &MathRuler;
    fn px_to_du(&self, px: f32) -> f32;
    fn sp_to_du(&self, sp: f32) -> f32;
    fn dp_to_du(&self, dp: f32) -> f32;
}