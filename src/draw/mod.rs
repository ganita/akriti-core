mod wrapper;                pub use self::wrapper::*;
mod text;                   pub use self::text::*;
mod absolute_layout;        pub use self::absolute_layout::*;
mod linear_layout;          pub use self::linear_layout::*;
mod fixed;                  pub use self::fixed::*;

use ::paint::{Canvas, Point, Rect};
use ::platform::Context;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    rect: Rect,
    baseline: f32,
    axis: f32
}

impl BoundingBox {
    pub fn new(rect: Rect, baseline: f32, axis: f32) -> BoundingBox {
        BoundingBox { rect, baseline, axis }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn baseline(&self) -> f32 {
        self.baseline
    }

    pub fn axis(&self) -> f32 {
        self.axis
    }

    pub fn width(&self) -> f32 {
        self.rect.width()
    }

    pub fn height(&self) -> f32 {
        self.rect.height()
    }

    pub fn ascent(&self) -> f32 { self.height()-self.baseline }

    pub fn descent(&self) -> f32 { -self.baseline }

    pub fn baseline_pos(&self) -> f32 {
        self.height()-self.baseline()
    }

    pub fn axis_pos(&self) -> f32 {
        self.height()-self.axis()
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            rect: Rect::new(0., 0.),
            baseline: 0.0,
            axis: 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MeasureMode {
    UpTo,
    Wrap
}

pub trait Drawable {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point);
    fn calculate(&mut self, context: &Context, width: f32, width_mode: &MeasureMode, height: f32,
                 height_mode: &MeasureMode);

    fn bounding_box(&self) -> &BoundingBox;
}