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

use super::super::{Mmultiscripts, Mmultiscript, Element, ElementType, InheritedProps, StyleProps,
                   Family, InstanceId, ScriptsAndLimits, Presentation, PresentationPrivate, Mempty,
                   SpecifiedPresentationProps};
use ::platform::Context;
use ::layout::{Layout};

pub struct Msup {
    multiscript: Mmultiscripts
}

impl Msup {
    pub fn new(base: Box<Element>, superscript: Box<Element>) -> Msup {
        let mut multiscript = Mmultiscripts::new(base);
        multiscript.with_postscript(Mmultiscript {
            superscript,
            subscript: Box::new(Mempty::new()),
        });

        Msup {
            multiscript,
        }
    }

    pub fn with_base<'a>(&'a mut self, base: Box<Element>) -> &'a mut Msup {
        self.multiscript.with_base(base);
        self
    }

    pub fn base(&self) -> &Box<Element> {
        self.multiscript.base()
    }
}

impl Element for Msup {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        self.multiscript.layout(context, family, inherited, style)
    }

    fn type_info(&self) -> ElementType {
        ElementType::ScriptsAndLimits(ScriptsAndLimits::Msup)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        self.multiscript.instance_id()
    }
}

impl PresentationPrivate<Mmultiscripts> for Msup {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        self.multiscript.get_specified_presentation_props()
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        self.multiscript.get_specified_presentation_props_mut()
    }
}

impl Presentation<Mmultiscripts> for Msup {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snapshot = Snapshot::default();

        let msub = Msup::new(
            Box::new(Mi::new(String::from("a"))),
            Box::new(Mi::new(String::from("3")))
        );

        snapshot.snap_element(&msub, "msup_simple");

        let msub = Msup::new(
            Box::new(Mi::new(String::from("a"))),
            Box::new(Msup::new(
                Box::new(Mi::new(String::from("a"))),
                Box::new(Msup::new(
                    Box::new(Mi::new(String::from("a"))),
                    Box::new(Msup::new(
                        Box::new(Mi::new(String::from("a"))),
                        Box::new(Mi::new(String::from("3")))
                    ))
                ))
            ))
        );

        snapshot.snap_element(&msub, "msup_nested");
    }
}