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


mod mi;                         pub use self::mi::*;
mod mn;                         pub use self::mn::*;
mod mtext;                      pub use self::mtext::*;
mod mo;                         pub use self::mo::*;
mod ms;                         pub use self::ms::*;
mod mspace;                     pub use self::mspace::*;

use std::rc::Rc;

use super::{ConcreteLayout, Layout, PresentationLayout};
use ::draw::{Text, Drawable, Wrapper, MeasureMode};
use ::platform::{Context};
use ::props::{MathVariant, Directionality, Color};


pub struct TokenLayout {
    pub(crate) text: Rc<String>,
    pub(crate) math_variant: MathVariant,
    pub(crate) math_size: f32,
    pub(crate) dir: Directionality,

    pub(crate) presentation_element: PresentationLayout
}

impl Layout for TokenLayout {
    fn layout<'a>(&'a self, _context: &Context) -> Box<Drawable + 'a> {
        unimplemented!()
    }
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, Text<'a, TokenLayout>>> for TokenLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, Text<'a, TokenLayout>> {
        let mut layout = self.presentation_element.layout(context);

        let text = Text::new(self, text_reader, math_size_reader,
                             math_variant_reader, dir_reader, math_color_reader);

        layout.wrap(text);
        layout.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        return layout;
    }
}

impl TokenLayout {
    pub fn new(text: String, math_variant: MathVariant, math_size: f32, dir: Directionality,
               math_color: Color, math_background: Color) -> TokenLayout {
        TokenLayout {
            text: Rc::new(text),
            math_variant,
            math_size,
            dir,
            presentation_element: PresentationLayout::new(math_color, math_background),
        }
    }
}

fn text_reader(element: &TokenLayout) -> &str {
    &element.text
}

fn math_variant_reader(element: &TokenLayout) -> &MathVariant {
    &element.math_variant
}

fn math_size_reader(element: &TokenLayout) -> f32 {
    element.math_size
}

fn dir_reader(element: &TokenLayout) -> &Directionality {
    &element.dir
}

fn math_color_reader(element: &TokenLayout) -> &Color {
    &element.presentation_element.math_color
}