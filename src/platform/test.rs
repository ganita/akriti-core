use std::any::Any;

use ::platform::{Platform, Context};
use ::paint::{TextRuler, MathRuler};
use ::elements::Element;

pub struct MockPlatform {}

impl Platform for MockPlatform {
    fn get_text_ruler(&self, _: &Element, _: f32) -> &TextRuler {
        unimplemented!()
    }

    fn get_math_ruler(&self, _: &Element, _: f32) -> &MathRuler {
        unimplemented!()
    }

    fn px_to_du(&self, _: f32) -> f32 {
        unimplemented!()
    }

    fn sp_to_du(&self, _: f32) -> f32 {
        unimplemented!()
    }

    fn dp_to_du(&self, _: f32) -> f32 {
        unimplemented!()
    }

    fn as_any(&self) -> &Any {
        self
    }
}

pub fn test_context() -> Context {
    Context::new(Box::new(MockPlatform {}), 12.)
}