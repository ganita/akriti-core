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


use std::borrow::Cow;

use super::{Drawable, BoundingBox, MeasureMode};
use ::props::{MathVariant, Directionality, Color};
use ::paint::{Canvas, Point, Rect};
use ::platform::Context;
use ::layout::Element;

pub type TextReader<T> = fn(&T) -> &str;
pub type MathSizeReader<T> = fn(&T) -> f32;
pub type MathVariantReader<T> = fn(&T) -> &MathVariant;
pub type DirectionalityReader<T> = fn(&T) -> &Directionality;
pub type MathColorReader<T> = fn(&T) -> &Color;

pub struct Text<'a, T: Element + 'a> {
    props: &'a T,

    text_reader: TextReader<T>,
    math_size_reader: MathSizeReader<T>,
    math_variant_reader: MathVariantReader<T>,
    dir_reader: DirectionalityReader<T>,
    math_color_reader: MathColorReader<T>,

    bounding_box: BoundingBox,

    variant_text: Cow<'a, str>
}

impl<'a, T: Element> Drawable for Text<'a, T> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        canvas.draw_text(
            pen_pos,
            &self.bounding_box,
            self.variant_text.as_ref(),
            (self.math_color_reader)(self.props),
            (self.math_size_reader)(self.props),
            (self.dir_reader)(self.props)
        )
    }

    fn calculate(&mut self, context: &Context, _: &MeasureMode, _: &MeasureMode) {
        let size = (self.math_size_reader)(self.props);

        let text = (self.text_reader)(self.props);

        self.variant_text = get_variant_text(
            context,
            text,
            (self.math_variant_reader)(self.props)
        );

        let ruler = context.platform().get_math_ruler(self.props, size);
        let width = ruler.measure(self.variant_text.as_ref(), (self.dir_reader)(self.props)).width();
        let height = ruler.ascent() - ruler.descent();
        let baseline = ruler.descent().abs();
        let axis = ruler.axis_height() + baseline;

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline,
            axis,
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

fn get_variant_text<'a>(context: &Context, text: &'a str, math_variant: &MathVariant) -> Cow<'a, str> {
    if *math_variant == MathVariant::Normal {
        return Cow::Borrowed(text);
    }

    let variant_dict = context.math_variants_dictionary();
    let variant = text.chars()
        .map(|c| if let Some(val) = variant_dict.variant_for_char(&c, math_variant) { val } else { c })
        .collect::<String>();

    return Cow::Owned(variant);
}

impl<'a, T: Element> Text<'a, T> {
    pub fn new(props: &'a T, text_reader: TextReader<T>, math_size_reader: MathSizeReader<T>,
               math_variant_reader: MathVariantReader<T>, dir_reader: DirectionalityReader<T>,
               math_color_reader: MathColorReader<T>) -> Text<'a, T> {
        Text {
            props,
            text_reader,
            math_size_reader,
            math_variant_reader,
            dir_reader,
            math_color_reader,
            bounding_box: BoundingBox::default(),
            variant_text: Cow::default(),
        }
    }
}