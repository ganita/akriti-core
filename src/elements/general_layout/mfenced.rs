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

use super::super::{Element, ElementType, GeneralLayout, InheritedProps, StyleProps, Family, InstanceId,
                   Presentation, PresentationPrivate, SpecifiedPresentationProps,
                   PropertyCalculator, Property, EmptyComputeCtx, Mrow, Mo};
use ::platform::Context;
use ::layout::{Layout};
use ::props::{Directionality};

#[allow(const_err)]
const PROP_DIRECTIONALITY: Property<Directionality, Mfenced, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.dir(),
    writer: |v, fork| fork.dir(v)
};

#[allow(const_err)]
const PROP_OPEN: Property<String, Mfenced, EmptyComputeCtx> = Property::Specified {
    default: || String::from("("),
    reader: |i| i.mfenced_open()
};

#[allow(const_err)]
const PROP_CLOSE: Property<String, Mfenced, EmptyComputeCtx> = Property::Specified {
    default: || String::from(")"),
    reader: |i| i.mfenced_close(),
};

#[allow(const_err)]
const PROP_SEPARATORS: Property<String, Mfenced, EmptyComputeCtx> = Property::Specified {
    default: || String::from(","),
    reader: |i| i.mfenced_separators(),
};

pub struct Mfenced {
    open: Option<String>,
    close: Option<String>,
    separators: Option<String>,

    children: Mrow,

    instance_id: InstanceId,
}

impl Mfenced {
    pub fn new() -> Mfenced {
        let mut mrow = Mrow::new();

        mrow.with_child(Box::new(Mo::new(String::from("("))));  // Placeholder for opening fence
        mrow.with_child(Box::new(Mrow::new()));                      // Placeholder for contents
        mrow.with_child(Box::new(Mo::new(String::from("("))));  // Placeholder for closing fence

        Mfenced {
            open: None,
            close: None,
            separators: None,
            children: mrow,

            instance_id: InstanceId::new(),
        }
    }

    pub fn with_child<'a>(&'a mut self, child: Box<Element>) -> &'a mut Mfenced {
        {
            let content: &mut Mrow =
                self.children.children_mut()
                    .get_mut(1)
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<Mrow>()
                    .unwrap();

            if content.children().len() > 0 {
                content.with_child(Box::new(Mo::new(String::from(","))));       // Placeholder separator
            }

            content.with_child(child);
        }

        self
    }
}

impl Element for Mfenced {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let open = calculator.calculate(&PROP_OPEN, self.open.as_ref());
        let close = calculator.calculate(&PROP_CLOSE, self.close.as_ref());

        let mut separators: Vec<char> = calculator.calculate(&PROP_SEPARATORS, self.close.as_ref())
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();


        unimplemented!()

    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Mfenced)
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

impl PresentationPrivate<Mfenced> for Mfenced {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        self.children.get_specified_presentation_props()
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        self.children.get_specified_presentation_props_mut()
    }
}

impl Presentation<Mfenced> for Mfenced {}