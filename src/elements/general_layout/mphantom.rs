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

use super::super::{Element, ElementType, GeneralLayout, InstanceId, InheritedProps, StyleProps,
                   Family, Presentation, PresentationPrivate, SpecifiedPresentationProps, PropertyCalculator};
use ::platform::Context;
use ::layout::{Layout, MphatomLayout};

pub struct Mphantom {
    child: Box<Element>,

    presentation_props: SpecifiedPresentationProps,

    instance_id: InstanceId,
}

impl Mphantom {
    pub fn new(child: Box<Element>) -> Mphantom {
        Mphantom {
            child,
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub fn with_child<'a>(&'a mut self, child: Box<Element>) -> &'a mut Mphantom {
        self.child = child;
        self
    }

    pub fn child(&self) -> &Box<Element> {
        &self.child
    }
}

impl Element for Mphantom {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);

        let new_family = family.add(self);
        let fork = calculator.make_fork().copy();

        Box::new(MphatomLayout {
            presentation_layout,
            child_layout: self.child.layout(context, &new_family, &fork, style)
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Mphantom)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mphantom> for Mphantom {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mphantom> for Mphantom {}

#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;
    use ::props::*;

    #[test]
    fn it_works() {
        let mut num = Mrow::new();
        num.with_child(Box::new(Mi::new(String::from("x"))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mi::new(String::from("y"))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mi::new(String::from("z"))));

        let mut phantom = Mrow::new();
        let mut phantom_op = Mo::new(String::from("+"));
        phantom_op.with_form(Some(OperatorForm::Infix));
        phantom.with_child(Box::new(phantom_op))
            .with_child(Box::new(Mi::new(String::from("y"))));

        let mut denom = Mrow::new();
        denom.with_child(Box::new(Mi::new(String::from("x"))))
            .with_child(Box::new(Mphantom::new(Box::new(phantom))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mi::new(String::from("z"))));

        Snapshot::default().snap_element(
            &Mfrac::new(Box::new(num), Box::new(denom)), "mphantom");
    }
}