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

use super::{TokenLayout, PresentationLayout};
use super::super::{Layout, ConcreteLayout};
use ::props::{MathVariant, Directionality, Color};
use ::platform::Context;
use ::draw::{Drawable, Wrapper, Text};

pub struct MiLayout {
    pub(crate) token_element: TokenLayout
}

impl MiLayout {
    pub fn new(text: String, math_variant: MathVariant, math_size: f32, dir: Directionality,
               math_color: Color, math_background: Color) -> MiLayout {
        MiLayout {
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

impl Layout for MiLayout {
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

impl<'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, Text<'a, TokenLayout>>> for MiLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationLayout, Text<'a, TokenLayout>> {
        <TokenLayout as ConcreteLayout<'a, Wrapper<'a, PresentationLayout, Text<'a, TokenLayout>>>>
        ::layout(&self.token_element, context)
    }
}
