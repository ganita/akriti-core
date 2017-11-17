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

use std::any::Any;

use ::props::{
    MathVariant,
    Directionality,
    Color
};
use super::{Layout, ConcreteLayout, TokenLayout};
use ::draw::{PaddingBox, Drawable, Symbol, MeasureMode};
use ::platform::Context;

pub struct MoLayout {
    pub(crate) lspace: f32,
    pub(crate) rspace: f32,
    pub(crate) stretchy: bool,
    pub(crate) symmetric: bool,
    pub(crate) max_size: f32,
    pub(crate) min_size: f32,
    pub(crate) large_op: bool,
    pub(crate) movable_limits: bool,
    pub(crate) accent: bool,

    pub(crate) token_element: TokenLayout,
}

impl Layout for MoLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteLayout::layout(self, context))
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl<'a> ConcreteLayout<'a, PaddingBox<'a, MoLayout, Symbol<'a, MoLayout>>> for MoLayout {
    fn layout(&'a self, context: &Context) -> PaddingBox<'a, MoLayout, Symbol<'a, MoLayout>> {
        let mut padding_box = PaddingBox::new(
            self,
            |s| s.lspace,
            |s| s.rspace,
            |_| 0.,
            |_| 0.,
        );

        let symbol = Symbol::new(
            self,
            |s| &s.token_element.text,
            |s| &s.token_element.math_variant,
            |s| s.symmetric,
            |s| s.token_element.math_size,
            |s| s.max_size,
            |s| s.min_size,
            |s| &s.token_element.dir,
            |s| &s.token_element.presentation_element.math_color,
        );

        padding_box.wrap(symbol);

        padding_box.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        padding_box
    }
}

impl MoLayout {
    pub fn new(
        text: String,
        math_variant: MathVariant,
        math_size: f32,
        dir: Directionality,
        math_color: Color,

        lspace: f32,
        rspace: f32,
        stretchy: bool,
        symmetric: bool,
        max_size: f32,
        min_size: f32,
        large_op: bool,
        movable_limits: bool,
        accent: bool,

        math_background: Color,
    ) -> MoLayout {
        MoLayout {
            lspace,
            rspace,
            stretchy,
            symmetric,
            max_size,
            min_size,
            large_op,
            movable_limits,
            accent,

            token_element: TokenLayout::new(
                text,
                math_variant,
                math_size,
                dir,
                math_color,
                math_background,
            )
        }
    }
}