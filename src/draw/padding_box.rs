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
use ::paint::{Canvas, Point, Rect};
use ::platform::Context;

pub type PaddingReader<T> = fn (&T) -> f32;

pub struct PaddingBox<'a, T: 'a, U: Drawable> {
    wrapped: Option<U>,
    props: &'a T,
    padding_left_reader: PaddingReader<T>,
    padding_right_reader: PaddingReader<T>,
    padding_top_reader: PaddingReader<T>,
    padding_bottom_reader: PaddingReader<T>,

    bounding_box: BoundingBox
}

impl<'a, T, U: Drawable> Drawable for PaddingBox<'a, T, U> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        if let Some(ref wrapped) = self.wrapped {
            let point = pen_pos+&Point::new((self.padding_left_reader)(self.props),
                                            (self.padding_top_reader)(self.props));
            wrapped.draw(canvas, &point);
        }
    }

    fn calculate(&mut self, context: &Context, width: f32, width_mode: &MeasureMode, height: f32,
                 height_mode: &MeasureMode) {
        let padding_left = (self.padding_left_reader)(self.props);
        let padding_right = (self.padding_right_reader)(self.props);
        let padding_top = (self.padding_top_reader)(self.props);
        let padding_bottom = (self.padding_bottom_reader)(self.props);

        self.bounding_box = if let Some(ref mut val) = self.wrapped {
            val.calculate(context, width, width_mode, height, height_mode);
            BoundingBox {
                rect: Rect::new(val.bounding_box().width()+padding_left+padding_right,
                                val.bounding_box().height()+padding_top+padding_bottom),
                baseline: val.bounding_box().baseline()+padding_bottom,
                axis: val.bounding_box().axis()+padding_bottom,
            }
        } else {
            BoundingBox {
                rect: Rect::new(padding_left+padding_right, padding_top+padding_bottom),
                baseline: 0.0,
                axis: 0.0,
            }
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a, T, U: Drawable> PaddingBox<'a, T, U> {
    pub fn new(props: &'a T, padding_left_reader: PaddingReader<T>, padding_right_reader: PaddingReader<T>,
               padding_top_reader: PaddingReader<T>, padding_bottom_reader: PaddingReader<T>)
               -> PaddingBox<'a, T, U> {
        PaddingBox {
            wrapped: None,
            props,
            padding_left_reader,
            padding_right_reader,
            padding_top_reader,
            padding_bottom_reader,
            bounding_box: BoundingBox::default(),
        }
    }

    pub fn wrap(&mut self, drawable: U) {
        self.wrapped = Some(drawable);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::{Fixed, MeasureMode};
    use ::platform::test::test_context;

    struct PaddingProps;

    #[test]
    fn it_pads() {
        let content = Fixed::new(10., 20., 2., 5.);
        let props = PaddingProps {};
        let context = test_context();
        let mut wrapped = PaddingBox::new(
            &props,
            |_| 5.,
            |_| 3.,
            |_| 4.,
            |_| 2.
        );

        wrapped.calculate(&context, -1., &MeasureMode::Wrap,
                          -1., &MeasureMode::Wrap);

        assert_eq!(wrapped.bounding_box().width(), 8.);
        assert_eq!(wrapped.bounding_box().height(), 6.);
        assert_eq!(wrapped.bounding_box().baseline(), 0.);
        assert_eq!(wrapped.bounding_box().axis(), 0.);

        wrapped.wrap(content);
        wrapped.calculate(&context, -1., &MeasureMode::Wrap,
                          -1., &MeasureMode::Wrap);

        assert_eq!(wrapped.bounding_box().width(), 18.);
        assert_eq!(wrapped.bounding_box().height(), 26.);
        assert_eq!(wrapped.bounding_box().baseline(), 4.);
        assert_eq!(wrapped.bounding_box().axis(), 7.);
    }
}