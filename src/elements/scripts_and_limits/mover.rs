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

use super::super::{Moverover, Element, ElementType, InheritedProps, StyleProps,
                   Family, InstanceId, ScriptsAndLimits, Presentation, PresentationPrivate, Mempty,
                   SpecifiedPresentationProps};
use ::platform::Context;
use ::layout::{Layout};
use ::props::{Accent, HAlign};

pub struct Mover {
    munderover: Moverover
}

impl Mover {
    pub fn new(base: Box<Element>, overscript: Box<Element>) -> Mover {
        Mover {
            munderover: Moverover::new(base, overscript, Box::new(Mempty::new())),
        }
    }

    pub fn with_base<'a>(&'a mut self, base: Box<Element>) -> &'a mut Mover {
        self.munderover.with_base(base);
        self
    }

    pub fn base(&self) -> &Box<Element> {
        self.munderover.base()
    }

    pub fn with_overscript<'a>(&'a mut self, overscript: Box<Element>) -> &'a mut Mover {
        self.munderover.with_overscript(overscript);
        self
    }

    pub fn overscript(&self) -> &Box<Element> {
        &self.munderover.overscript()
    }

    pub fn with_accent<'a>(&'a mut self, accent: Option<Accent>) -> &'a mut Mover {
        self.munderover.with_accent(accent);
        self
    }

    pub fn accent(&self) -> Option<&Accent> {
        self.munderover.accent()
    }

    pub fn with_align<'a>(&'a mut self, align: Option<HAlign>) -> &'a mut Mover {
        self.munderover.with_align(align);
        self
    }

    pub fn align(&self) -> Option<&HAlign> {
        self.munderover.align()
    }
}

impl Element for Mover {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        self.munderover.layout(context, family, inherited, style)
    }

    fn type_info(&self) -> ElementType {
        ElementType::ScriptsAndLimits(ScriptsAndLimits::Mover)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        self.munderover.instance_id()
    }
}

impl PresentationPrivate<Moverover> for Mover {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        self.munderover.get_specified_presentation_props()
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        self.munderover.get_specified_presentation_props_mut()
    }
}

impl Presentation<Moverover> for Mover {}
