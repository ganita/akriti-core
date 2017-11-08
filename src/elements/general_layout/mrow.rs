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

use super::super::{Element, Presentation, PresentationPrivate, ElementType, TokenElement,
                   SpecifiedPresentationProps, InheritedProps, StyleProps, Property,
                   PropertyCalculator, InstanceId, Family};
use ::props::*;
use ::platform::Context;
use ::layout::{Layout, MrowLayout};

#[allow(const_err)]
const PROP_DIRECTIONALITY: Property<Directionality, Mrow> = Property::Inherited {
    reader: |i| i.dir(),
    writer: |v, fork| fork.dir(v)
};

pub struct Mrow {
    instance_id: InstanceId,

    children: Vec<Box<Element>>,

    dir: Option<Directionality>,
    presentation_props: SpecifiedPresentationProps
}

impl Mrow {
    pub fn new() -> Mrow {
        Mrow {
            instance_id: InstanceId::new(),
            children: Vec::new(),
            dir: None,
            presentation_props: SpecifiedPresentationProps::default(),
        }
    }

    pub fn with_dir<'a>(&'a mut self, dir: Option<Directionality>) -> &'a mut Mrow {
        self.dir = dir;
        self
    }

    pub fn get_dir(&self) -> Option<&Directionality> {
        self.dir.as_ref()
    }

    pub fn with_child<'a>(&'a mut self, child: Box<Element>) -> &'a mut Mrow {
        self.children.push(child);
        self
    }

    pub fn children(&self) -> &[Box<Element>] {
        &self.children[..]
    }
}

impl Element for Mrow {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let dir = calculator.calculate(&PROP_DIRECTIONALITY, self.get_dir());

        let fork = calculator.make_fork().copy();
        let new_family = family.add(self);

        Box::new(MrowLayout {
            presentation_element: presentation_layout,
            dir,
            elements: self.children.iter().map(|e|
                e.layout(context, &new_family, &fork, style)).collect(),
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mi)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mrow> for Mrow {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mrow> for Mrow {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::Mi;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(
            Mrow::new()
                .with_child(Box::new(Mi::new(String::from("i"))))
                .with_child(Box::new(Mi::new(String::from(" + "))))
                .with_child(Box::new(Mi::new(String::from("x")))),
            "mrow_ltr");

        snap.snap_element(
            Mrow::new()
                .with_child(Box::new(Mi::new(String::from("i"))))
                .with_child(Box::new(Mi::new(String::from(" + "))))
                .with_child(Box::new(Mi::new(String::from("x"))))
                .with_dir(Some(Directionality::RTL)),
            "mrow_rtl");

        snap.snap_element(
            Mrow::new()
                .with_child(Box::new(Mi::new(String::from("i"))))
                .with_child(Box::new(Mi::new(String::from(" + "))))
                .with_child(Box::new(Mi::new(String::from("x"))))
                .with_math_background(Some(Color::RGB(0, 255, 0))),
            "mrow_green_bg");

        snap.snap_element(
            Mrow::new()
                .with_child(Box::new(Mi::new(String::from("i"))))
                .with_child(Box::new(Mi::new(String::from(" + "))))
                .with_child(Box::new(Mi::new(String::from("x"))))
                .with_math_color(Some(Color::RGB(0, 255, 0))),
            "mrow_green_text");
    }
}