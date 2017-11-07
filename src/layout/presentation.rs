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


use ::props::{Color, DisplayStyle, ScriptLevel, ScriptMinSize, ScriptSizeMultiplier};
use super::ConcreteLayout;
use ::platform::Context;
use ::draw::{Drawable, Wrapper};

pub struct PresentationLayout {
    pub(crate) math_color: Color,
    pub(crate) math_background: Color,
    pub(crate) display_style: DisplayStyle,
    pub(crate) script_level: ScriptLevel,
    pub(crate) script_min_size: ScriptMinSize,
    pub(crate) script_size_multiplier: ScriptSizeMultiplier,
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


// TODO remove
impl PresentationLayout {
    pub fn new(math_color: Color, math_background: Color) -> PresentationLayout {
        PresentationLayout {
            math_color, math_background, display_style: false,
            script_level: ScriptLevel::new(0, 12.),
            script_min_size: 0.0, script_size_multiplier: 0.0
        }
    }
}