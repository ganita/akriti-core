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


use std::rc::Rc;

use super::{PresentationLayout};
use super::super::{Layout, ConcreteLayout};
use ::props::{LineBreak};
use ::platform::Context;
use ::draw::{Drawable, Wrapper, Space, MeasureMode};

pub struct MspaceLayout {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) depth: f32,
    pub(crate) _linebreak: LineBreak,

    pub(crate) presentation_layout: PresentationLayout
}

impl Layout for MspaceLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteLayout::layout(self, context))
    }
}

impl<'a> ConcreteLayout<'a, Wrapper<'a, MspaceLayout, Space>> for MspaceLayout {
    fn layout(&'a self, context: &Context) -> Wrapper<'a, MspaceLayout, Space> {
        let space = Space::new(self.width, self.height+self.depth, self.depth, self.depth);
        let mut wrapper = Wrapper::new(
            self,
            |mspace| &mspace.presentation_layout.math_background
        );

        wrapper.wrap(space);

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        wrapper
    }
}