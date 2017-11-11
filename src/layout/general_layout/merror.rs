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

use super::super::{Layout, PresentationLayout, ConcreteLayout};
use ::draw::{Drawable, MeasureMode, AbsoluteLayout, AbsoluteLayoutParams};
use ::platform::Context;
use ::paint::Point;


pub struct MerrorLayout {
    pub(crate) presentation_layout: PresentationLayout,
    pub(crate) child_layout: Box<Layout>,
}

impl Layout for MerrorLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        let mut wrapper = self.presentation_layout.layout(context);

        let child_drawable = self.child_layout.layout(context);

        let mut ll = AbsoluteLayout::new();
        ll.add_child(child_drawable, AbsoluteLayoutParams::new(Point::new(0., 0.)));
        wrapper.wrap(ll);

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