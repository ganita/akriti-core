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
use super::ConcreteElement;
use ::platform::Context;
use ::draw::{Drawable, Wrapper};

pub struct PresentationElement {
    pub math_background: Color
}

fn math_background_reader(element: &PresentationElement) -> &Color {
    &element.math_background
}

impl<'a, U: Drawable + 'a> ConcreteElement<'a, Wrapper<'a, PresentationElement, U>> for PresentationElement {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, PresentationElement, U> {
        Wrapper::<'a, PresentationElement, U>::new(
            self,
            math_background_reader
        )
    }
}

impl PresentationElement {
    pub fn new(math_background: Color) -> PresentationElement {
        PresentationElement { math_background }
    }
}