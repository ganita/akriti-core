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

use super::super::{Element, Presentation, PresentationPrivate, ElementType, GeneralLayout,
                   SpecifiedPresentationProps, InheritedProps, StyleProps, Property,
                   PropertyCalculator, InstanceId, Family, EmptyComputeCtx};
use ::props::*;
use ::platform::Context;
use ::layout::{Layout, MrootLayout};

#[allow(const_err)]
const PROP_DIRECTIONALITY: Property<Directionality, Mroot, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.dir(),
    writer: |v, fork| fork.dir(v)
};

pub struct Mroot {
    instance_id: InstanceId,

    child: Box<Element>,
    degree: Box<Element>,

    dir: Option<Directionality>,
    presentation_props: SpecifiedPresentationProps
}

impl Mroot {
    pub fn new(child: Box<Element>, degree: Box<Element>) -> Mroot {
        Mroot {
            instance_id: InstanceId::new(),

            child,
            degree,

            dir: None,
            presentation_props: SpecifiedPresentationProps::default(),
        }
    }

    pub fn with_dir<'a>(&'a mut self, dir: Option<Directionality>) -> &'a mut Mroot {
        self.dir = dir;
        self
    }

    pub fn get_dir(&self) -> Option<&Directionality> {
        self.dir.as_ref()
    }

    pub fn with_child<'a>(&'a mut self, element: Box<Element>) -> &'a mut Mroot {
        self.child = element;
        self
    }

    pub fn get_child(&self) -> &Box<Element> {
        &self.child
    }

    pub fn with_degree<'a>(&'a mut self, element: Box<Element>) -> &'a mut Mroot {
        self.degree = element;
        self
    }

    pub fn get_degree(&self) -> &Box<Element> {
        &self.degree
    }
}

impl Element for Mroot {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let dir = calculator.calculate(&PROP_DIRECTIONALITY, self.get_dir());

        let fork = calculator.make_fork().copy();
        let new_family = family.add(self);

        let base_size = presentation_layout.script_level.get_font_size(
            context, &MathSize::NORMAL);

        // For degree, increment script level by two and make display style false
        let mut inherited_props_copier = fork.copier();
        inherited_props_copier
            .display_style(false)
            .script_level(presentation_layout.script_level.new_level(
                PropertyModifier::Increment(2), context, &MathSize::NORMAL,
                presentation_layout.script_size_multiplier, presentation_layout.script_min_size)
            );
        let degree_inherited_props = inherited_props_copier.copy();

        Box::new(MrootLayout {
            child: self.child.layout(context, &new_family, &fork, style),
            degree: self.degree.layout(context, &new_family, &degree_inherited_props, style),
            base_size,

            presentation_element: presentation_layout,
            dir,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Mroot)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mroot> for Mroot {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mroot> for Mroot {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(&Mroot::new(
            Box::new(Mi::new(String::from("x"))),
            Box::new(Mn::new(String::from("3")))
        ), "mroot_simple");


        let mut test_row = Mrow::new();
        test_row.with_child(Box::new(Mi::new(String::from("k"))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mn::new(String::from("3"))));

        snap.snap_element(&Mroot::new(
            Box::new(Mi::new(String::from("x"))),
            Box::new(Mfrac::new(
                Box::new(Mn::new(String::from("3"))),
                Box::new(test_row)
            ))
        ), "mroot_frac_degree");

        let mut test_row = Mrow::new();
        test_row.with_child(Box::new(Mi::new(String::from("k"))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mn::new(String::from("3"))));

        snap.snap_element(&Mroot::new(
            Box::new(Mfrac::new(
                Box::new(Mn::new(String::from("3"))),
                Box::new(test_row)
            )),
            Box::new(Mi::new(String::from("x"))),
        ), "mroot_frac_child");
    }

    #[test]
    fn test_nested() {
        let snap = Snapshot::default();

        let create_nested = | element: Box<Element> | -> Box<Element> {
            let mut mrow = Mrow::new();
            mrow.with_child(Box::new(Mn::new(String::from("1"))))
                .with_child(Box::new(Mo::new(String::from("+"))))
                .with_child(element);

            Box::new(Mroot::new(Box::new(mrow), Box::new(Mn::new(String::from("3")))))
        };

        snap.snap_element(create_nested(
            create_nested(
                create_nested(
                    create_nested(
                        create_nested(
                            create_nested(
                                create_nested(Box::new(Mi::new(String::from("x")))))))))
        ).as_ref(), "mroot_nested")

    }
}