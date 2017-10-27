use super::{Drawable, BoundingBox, MeasureMode};
use ::paint::{Rect, Point, Canvas};
use ::platform::Context;
use ::props::Color;

pub struct Fixed {
    bounding_box: BoundingBox,
    pub background: Color,
    pub flex: bool,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub axis: f32,
}

impl Drawable for Fixed {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        canvas.draw_rect(
            pen_pos,
            self.bounding_box.rect(),
            &self.background
        );
        canvas.draw_rect_outline(
            pen_pos,
            self.bounding_box.rect(),
            &Color::RGB(255, 255, 255),
            1.
        );
    }

    fn calculate(&mut self, _: &Context, width: f32, width_measure_mode: &MeasureMode,
                 height: f32, height_measure_mode: &MeasureMode) {
        let width = if self.flex && *width_measure_mode == MeasureMode::UpTo {
            width.max(self.width)
        } else {
            self.width
        };

        let height = if self.flex && *height_measure_mode == MeasureMode::UpTo {
            height.max(self.height)
        } else {
            self.height
        };

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: self.baseline,
            axis: self.axis,
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl Fixed {
    pub fn new(width: f32, height: f32, baseline: f32, axis: f32) -> Fixed {
        Fixed {
            bounding_box: BoundingBox::default(),
            background: Color::RGB(0, 0, 0),
            flex: false,
            width,
            height,
            baseline,
            axis,
        }
    }
}