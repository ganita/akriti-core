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
use ::platform::Context;
use ::draw::{Drawable, Wrapper, Text, LinearLayout, Gravity, Align, LinearLayoutParams, MeasureMode};

pub struct MsLayout {
    pub(crate) token_element: TokenLayout,
    pub(crate) lquote: String,
    pub(crate) rquote: String,
}

impl Layout for MsLayout {
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

impl<'a> ConcreteLayout<'a, Wrapper<'a, MsLayout, LinearLayout<'a>>> for MsLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, MsLayout, LinearLayout<'a>> {
        let token_layout =
            <TokenLayout as ConcreteLayout<'a, Wrapper<'a, PresentationLayout, Text<'a, TokenLayout>>>>
            ::layout(&self.token_element, context);

        let lquote_layout = Text::new(
            self,
            |ms| &ms.lquote,
            |ms| ms.token_element.math_size,
            |ms| &ms.token_element.math_variant,
            |ms| &ms.token_element.dir,
            |ms| &ms.token_element.presentation_element.math_color
        );

        let rquote_layout = Text::new(
            self,
            |ms| &ms.rquote,
            |ms| ms.token_element.math_size,
            |ms| &ms.token_element.math_variant,
            |ms| &ms.token_element.dir,
            |ms| &ms.token_element.presentation_element.math_color
        );

        let mut layout = LinearLayout::new();
        layout.gravity = Gravity::Horizontal;
        layout.layout_align = Align::Baseline;

        layout.add_child(Box::new(lquote_layout), LinearLayoutParams::new());
        layout.add_child(Box::new(token_layout), LinearLayoutParams::new());
        layout.add_child(Box::new(rquote_layout), LinearLayoutParams::new());

        let mut wrapper = Wrapper::new(
            self,
            |ms| &ms.token_element.presentation_element.math_background,
        );

        wrapper.wrap(layout);

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        wrapper
    }
}
