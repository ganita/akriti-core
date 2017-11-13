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

use super::super::{Munderover, Element, ElementType, InheritedProps, StyleProps,
                   Family, InstanceId, ScriptsAndLimits, Presentation, PresentationPrivate, Mempty,
                   SpecifiedPresentationProps};
use ::platform::Context;
use ::layout::{Layout};
use ::props::{Accent, HAlign};

pub struct Munder {
    munderover: Munderover
}

impl Munder {
    pub fn new(base: Box<Element>, underscript: Box<Element>) -> Munder {
        Munder {
            munderover: Munderover::new(base, Box::new(Mempty::new()), underscript),
        }
    }

    pub fn with_base<'a>(&'a mut self, base: Box<Element>) -> &'a mut Munder {
        self.munderover.with_base(base);
        self
    }

    pub fn base(&self) -> &Box<Element> {
        self.munderover.base()
    }

    pub fn with_underscript<'a>(&'a mut self, underscript: Box<Element>) -> &'a mut Munder {
        self.munderover.with_underscript(underscript);
        self
    }

    pub fn underscript(&self) -> &Box<Element> {
        &self.munderover.underscript()
    }

    pub fn with_accent_under<'a>(&'a mut self, accent_under: Option<Accent>) -> &'a mut Munder {
        self.munderover.with_accent_under(accent_under);
        self
    }

    pub fn accent_under(&self) -> Option<&Accent> {
        self.munderover.accent_under()
    }

    pub fn with_align<'a>(&'a mut self, align: Option<HAlign>) -> &'a mut Munder {
        self.munderover.with_align(align);
        self
    }

    pub fn align(&self) -> Option<&HAlign> {
        self.munderover.align()
    }
}

impl Element for Munder {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        self.munderover.layout(context, family, inherited, style)
    }

    fn type_info(&self) -> ElementType {
        ElementType::ScriptsAndLimits(ScriptsAndLimits::Munder)
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

impl PresentationPrivate<Munderover> for Munder {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        self.munderover.get_specified_presentation_props()
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        self.munderover.get_specified_presentation_props_mut()
    }
}

impl Presentation<Munderover> for Munder {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;

    #[ignore]
    #[test]
    fn it_works() {
        let snapshot = Snapshot::default();
        let mut row = Mrow::new();
        row.with_child(Box::new(Mi::new(String::from("a"))));
        row.with_child(Box::new(Mo::new(String::from("+"))));
        row.with_child(Box::new(Mi::new(String::from("x"))));
        row.with_child(Box::new(Mi::new(String::from("i"))));

        let munder = Munder::new(Box::new(row), Box::new(Mo::new(String::from("\u{23b5}"))));

        snapshot.snap_element(&munder, "munder_simple");

        let mut row = Mrow::new();
        row.with_child(Box::new(Mi::new(String::from("a"))));
        row.with_child(Box::new(Mo::new(String::from("+"))));
        row.with_child(Box::new(Mi::new(String::from("x"))));
        row.with_child(Box::new(Mi::new(String::from("i"))));

        let mut underscript = Mrow::new();
        underscript.with_child(Box::new(Mi::new(String::from("x"))));
        underscript.with_child(Box::new(Mo::new(String::from("→"))));
        underscript.with_child(Box::new(Mi::new(String::from("∞"))));

        let munder = Munder::new(Box::new(row), Box::new(underscript));

        snapshot.snap_element(&munder, "munder_weight_stretched");
    }
}