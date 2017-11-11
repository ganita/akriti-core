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

use super::super::{Mroot, Mempty, Element, ElementType, GeneralLayout, InheritedProps,
                   StyleProps, Family, InstanceId, Presentation, PresentationPrivate,
                   SpecifiedPresentationProps};
use ::platform::Context;
use ::layout::{Layout};


pub struct Msqrt {
    root: Mroot
}

impl Msqrt {
    pub fn new(child: Box<Element>) -> Msqrt {
        Msqrt {
            root: Mroot::new(child, Box::new(Mempty::new())),
        }
    }
}

impl Element for Msqrt {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        self.root.layout(context, family, inherited, style)
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Msqrt)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        self.root.instance_id()
    }
}

impl PresentationPrivate<Msqrt> for Msqrt {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        self.root.get_specified_presentation_props()
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        self.root.get_specified_presentation_props_mut()
    }
}

impl Presentation<Msqrt> for Msqrt {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;

    #[test]
    fn test_nested() {
        let snap = Snapshot::default();

        let create_nested = | element: Box<Element> | -> Box<Element> {
            let mut mrow = Mrow::new();
            mrow.with_child(Box::new(Mn::new(String::from("1"))))
                .with_child(Box::new(Mo::new(String::from("+"))))
                .with_child(element);

            Box::new(Msqrt::new(Box::new(mrow)))
        };

        snap.snap_element(create_nested(
            create_nested(
                create_nested(
                    create_nested(
                        create_nested(
                            create_nested(
                                create_nested(Box::new(Mi::new(String::from("x")))))))))
        ).as_ref(), "msqrt_nested")

    }
}