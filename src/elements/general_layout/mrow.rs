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
                   SpecifiedPresentationProps, InheritedProps, StyleProps, Property};
use ::props::*;
use ::platform::Context;
use ::layout::{Layout, MrowLayout};

const PROP_DIRECTIONALITY: Property<Directionality, Mrow> = Property::Inherited {
    reader: |i| i.dir()
};

pub struct Mrow {
    children: Vec<Box<Element>>,

    dir: Option<Directionality>,
    presentation_props: SpecifiedPresentationProps
}

impl Mrow {
    pub fn new() -> Mrow {
        Mrow {
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
}

impl Element for Mrow {
    fn layout(&self, context: &Context, parent: Option<&Element>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        Box::new(MrowLayout {
            elements: self.children.iter().map(|e|
                e.layout(context, Some(self), inherited, style)).collect(),
            dir: PROP_DIRECTIONALITY.calculate(
                context, self, self.get_dir(), &parent, inherited, style),
            presentation_element: self.layout_presentation(
                self, context, Some(self), inherited, style),
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mi)
    }

    fn as_any(&self) -> &Any {
        self
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