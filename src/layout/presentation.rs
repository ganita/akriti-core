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


use ::props::Color;
use super::ConcreteLayout;
use ::platform::Context;
use ::draw::{Drawable, Wrapper};

pub struct PresentationLayout {
    pub math_color: Color,
    pub math_background: Color,
}

fn math_background_reader(element: &PresentationLayout) -> &Color {
    &element.math_background
}

impl<'a, U: Drawable + 'a> ConcreteLayout<'a, Wrapper<'a, PresentationLayout, U>> for PresentationLayout {
    fn layout(&'a self, _: &Context) -> Wrapper<'a, PresentationLayout, U> {
        Wrapper::<'a, PresentationLayout, U>::new(
            self,
            math_background_reader
        )
    }
}

impl PresentationLayout {
    pub fn new(math_color: Color, math_background: Color) -> PresentationLayout {
        PresentationLayout { math_color, math_background }
    }
}