use super::{Drawable, BoundingBox, MeasureMode};
use ::props::Color;
use ::paint::{Canvas, Point, Rect};
use ::platform::Context;

pub type MathBackgroundReader<T> = fn (&T) -> &Color;

pub struct Wrapper<'a, T: 'a, U: Drawable> {
    wrapped: Option<U>,
    props: &'a T,
    math_background_reader: MathBackgroundReader<T>,

    bounding_box: BoundingBox
}

impl<'a, T, U: Drawable> Drawable for Wrapper<'a, T, U> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        canvas.draw_rect(
            pen_pos,
            self.bounding_box.rect(),
            (self.math_background_reader)(self.props)
        );
        if let Some(ref wrapped) = self.wrapped {
            wrapped.draw(canvas, pen_pos);
        }
    }

    fn calculate(&mut self, context: &Context, width: f32, width_mode: &MeasureMode, height: f32,
                 height_mode: &MeasureMode) {
        if let Some(val) = self.wrapped.as_mut() {
            val.calculate(context, width, width_mode, height, height_mode);
        }
        self.bounding_box = BoundingBox {
            rect: if let Some(ref val) = self.wrapped {
                val.bounding_box().rect().clone()
            } else {
                Rect::new(0., 0.)
            },
            baseline: if let Some(ref val) = self.wrapped {
                val.bounding_box().baseline()
            } else {
                0.
            },
            axis: if let Some(ref val) = self.wrapped {
                val.bounding_box().axis()
            } else {
                0.
            },
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a, T, U: Drawable> Wrapper<'a, T, U> {
    pub fn new(props: &'a T, math_background_reader: MathBackgroundReader<T>)
               -> Wrapper<'a, T, U> {
        Wrapper {
            wrapped: None,
            props,
            math_background_reader,
            bounding_box: BoundingBox::default()
        }
    }

    pub fn wrap(&mut self, drawable: U) {
        self.wrapped = Some(drawable);
    }
}