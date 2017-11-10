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


use super::{Drawable, BoundingBox, MeasureMode};
use ::layout::{Layout};
use ::paint::{Canvas, Point, Rect};
use ::platform::Context;
use ::props::{Color};

pub type ColorReader<T> = fn(&T) -> &Color;

pub enum LineParam {
    Fixed { start: Point, end: Point },
    Vertical { x: f32 },
    Horizontal { y: f32 },
}

pub struct Line<'a, T: Layout + 'a> {
    param: LineParam,

    element: &'a T,
    stroke_width: f32,
    color_reader: ColorReader<T>,

    bounding_box: BoundingBox,
    start: Point,
    end: Point,
}

impl<'a, T: Layout + 'a> Drawable for Line<'a, T> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        canvas.draw_line(
            &(&self.start+pen_pos),
            &(&self.end+pen_pos),
            (self.color_reader)(self.element),
            self.stroke_width,
        )
    }

    fn calculate(&mut self, _: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        let (start, end) = match self.param {
            LineParam::Fixed { ref start, ref end } => {
                (start.clone(), end.clone())
            },
            LineParam::Vertical { x } => {
                (
                    Point::new(x, 0.),
                    Point::new(x, if let MeasureMode::UpTo(height) = *height_mode { height } else { 0. })
                )
            },
            LineParam::Horizontal { y } => {
                (
                    Point::new(0., y),
                    Point::new(if let MeasureMode::UpTo(width) = *width_mode { width } else { 0. }, y)
                )
            }
        };

        let stroke_width = self.stroke_width;

        let slope = (start.y()-end.y()).abs() / (start.x()-end.x()).abs();
        let angle = slope.atan();
        let x_diff = angle.sin()*stroke_width/2.;
        let y_diff = angle.cos()*stroke_width/2.;

        let mut width = (start.x()-end.x()).abs()+(x_diff*2.);
        let mut height = (start.y()-end.y()).abs()+(y_diff*2.);

        if width.is_nan() {
            width = 0.;
        }

        if height.is_nan() {
            height = 0.;
        }

        self.start = &start+&Point::new(x_diff, y_diff+(stroke_width/2.));
        self.end = &end+&Point::new(x_diff, y_diff+(stroke_width/2.));

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: height/2.,
            axis: height/2.,
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a, T: Layout + 'a> Line<'a, T> {
    pub fn new(param: LineParam, element: &'a T, stroke_width: f32,
               color_reader: ColorReader<T>) -> Line<'a, T> {
        Line {
            param,
            element,
            stroke_width,
            color_reader,
            bounding_box: BoundingBox::default(),
            start: Point::new(0., 0.),
            end: Point::new(0., 0.),
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;
    use ::platform::test::test_context;
    use ::test::skia::Snapshot;

    struct MockElement;
    impl Layout for MockElement {
        fn layout<'a>(&'a self, _: &Context) -> Box<Drawable + 'a> {
            unimplemented!()
        }

        fn as_any(&self) -> &Any {
            unimplemented!()
        }
    }

    #[test]
    fn it_work_vertically() {
        let context = test_context();
        let element = MockElement {};
        let mut line = Line::new(
            LineParam::Vertical { x: 10. },
            &element,
            5.,
            |_| &Color::RGB(0, 0, 0)
        );

        line.calculate(&context, &MeasureMode::Wrap, &MeasureMode::UpTo(100.));

        assert_eq!(line.bounding_box().width(), 5.);
        assert_eq!(line.bounding_box().height(), 100.);
    }

    #[test]
    fn it_work_horizontally() {
        let context = test_context();
        let element = MockElement {};
        let mut line = Line::new(
            LineParam::Horizontal { y: 10. },
            &element,
            5.,
            |_| &Color::RGB(0, 0, 0)
        );

        line.calculate(&context, &MeasureMode::UpTo(100.), &MeasureMode::Wrap);

        assert_eq!(line.bounding_box().width(), 100.);
        assert_eq!(line.bounding_box().height(), 5.);
    }

    #[test]
    fn it_works_when_inclined() {
        let context = test_context();
        let element = MockElement {};
        let mut line = Line::new(
            LineParam::Fixed { start: Point::new(0., 0.), end: Point::new(10., 10.) },
            &element,
            5.,
            |_| &Color::RGB(0, 0, 0)
        );

        line.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        assert_eq!(line.bounding_box().width(), 13.535534);
        assert_eq!(line.bounding_box().height(), 13.535534);
    }

    #[test]
    fn test_line() {
        let skia = Snapshot::default();

        let element = MockElement { };

        let mut line = Line::new(
            LineParam::Fixed { start: Point::new(0., 0.), end: Point::new(100., 100.) },
            &element,
            50.,
            |_| &Color::RGB(0, 0, 0)
        );

        skia.snap_drawable(&mut line, &MeasureMode::Wrap, &MeasureMode::Wrap,
                      "line_45deg");

        let mut line = Line::new(
            LineParam::Fixed { start: Point::new(0., 0.), end: Point::new(50., 100.) },
            &element,
            50.,
            |_| &Color::RGB(0, 0, 0)
        );

        skia.snap_drawable(&mut line, &MeasureMode::Wrap, &MeasureMode::Wrap,
                      "line_inclined");

        let mut line = Line::new(
            LineParam::Vertical { x: 0. },
            &element,
            50.,
            |_| &Color::RGB(0, 0, 0)
        );

        skia.snap_drawable(&mut line, &MeasureMode::Wrap, &MeasureMode::UpTo(100.),
                      "line_vertical");

        let mut line = Line::new(
            LineParam::Horizontal { y: 0. },
            &element,
            50.,
            |_| &Color::RGB(0, 0, 0)
        );

        skia.snap_drawable(&mut line, &MeasureMode::UpTo(100.), &MeasureMode::Wrap,
                      "line_horizontal");
    }
}