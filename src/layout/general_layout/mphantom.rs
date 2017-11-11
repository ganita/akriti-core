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

use super::super::{Layout, ConcreteLayout, PresentationLayout};
use ::draw::{Drawable, Phantom, MeasureMode};
use ::platform::Context;


pub struct MphatomLayout {
    pub(crate) presentation_layout: PresentationLayout,
    pub(crate) child_layout: Box<Layout>,
}

impl Layout for MphatomLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        let mut wrapper = self.presentation_layout.layout(context);
        let mut phantom = Phantom::new();

        let child_layout = self.child_layout.layout(context);
        phantom.wrap(Some(child_layout));

        wrapper.wrap(phantom);

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        Box::new(wrapper)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}