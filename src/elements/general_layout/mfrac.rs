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
                   PropertyCalculator, InstanceId, Family, DefaultComputationContext};
use ::props::*;
use ::platform::Context;
use ::layout::{Layout, MfracLayout};

#[allow(const_err)]
const PROP_DIRECTIONALITY: Property<Directionality, Mfrac, DefaultComputationContext> = Property::Inherited {
    reader: |i| i.dir(),
    writer: |v, fork| fork.dir(v)
};

#[allow(const_err)]
const PROP_LINE_THICKNESS: Property<LineThickness, Mfrac, DefaultComputationContext> = Property::Specified {
    default: || LineThickness::MEDIUM,
    reader: |s| s.mfrac_line_thickness(),
};

#[allow(const_err)]
const PROP_NUM_ALIGN: Property<HAlign, Mfrac, DefaultComputationContext> = Property::Specified {
    default: || HAlign::Center,
    reader: |s| s.mfrac_num_align(),
};

#[allow(const_err)]
const PROP_DENOM_ALIGN: Property<HAlign, Mfrac, DefaultComputationContext> = Property::Specified {
    default: || HAlign::Center,
    reader: |s| s.mfrac_denom_align(),
};

#[allow(const_err)]
const PROP_BEVELLED: Property<bool, Mfrac, DefaultComputationContext> = Property::Specified {
    default: || false,
    reader: |s| s.mfrac_bevelled(),
};

pub struct Mfrac {
    instance_id: InstanceId,

    numerator: Box<Element>,
    denominator: Box<Element>,

    line_thickness: Option<LineThickness>,
    num_align: Option<HAlign>,
    denom_align: Option<HAlign>,
    bevelled: Option<bool>,

    presentation_props: SpecifiedPresentationProps
}

impl Mfrac {
    pub fn new(numerator: Box<Element>, denominator: Box<Element>) -> Mfrac {
        Mfrac {
            instance_id: InstanceId::new(),

            numerator,
            denominator,

            line_thickness: None,
            num_align: None,
            denom_align: None,
            bevelled: None,
            
            presentation_props: SpecifiedPresentationProps::default(),
        }
    }

    pub fn with_numerator<'a>(&'a mut self, element: Box<Element>) -> &'a mut Mfrac {
        self.numerator = element;
        self
    }

    pub fn get_numerator(&self) -> &Box<Element> {
        &self.numerator
    }

    pub fn with_denominator<'a>(&'a mut self, element: Box<Element>) -> &'a mut Mfrac {
        self.denominator = element;
        self
    }

    pub fn get_denominator(&self) -> &Box<Element> {
        &self.denominator
    }

    pub fn with_line_thickness<'a>(&'a mut self, line_thickness: Option<LineThickness>) -> &'a mut Mfrac {
        self.line_thickness = line_thickness;
        self
    }

    pub fn get_line_thickness(&self) -> Option<&LineThickness> {
        self.line_thickness.as_ref()
    }

    pub fn with_num_align<'a>(&'a mut self, num_align: Option<HAlign>) -> &'a mut Mfrac {
        self.num_align = num_align;
        self
    }

    pub fn get_num_align(&self) -> Option<&HAlign> {
        self.num_align.as_ref()
    }

    pub fn with_denom_align<'a>(&'a mut self, denom_align: Option<HAlign>) -> &'a mut Mfrac {
        self.denom_align = denom_align;
        self
    }

    pub fn get_denom_align(&self) -> Option<&HAlign> {
        self.denom_align.as_ref()
    }

    pub fn with_bevelled<'a>(&'a mut self, bevelled: Option<bool>) -> &'a mut Mfrac {
        self.bevelled = bevelled;
        self
    }

    pub fn get_bevelled(&self) -> Option<&bool> {
        self.bevelled.as_ref()
    }
}

impl Element for Mfrac {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let dir = calculator.calculate(&PROP_DIRECTIONALITY, None);
        let line_thickness = calculator.calculate(
            &PROP_LINE_THICKNESS, self.line_thickness.as_ref());
        let num_align = calculator.calculate(
            &PROP_NUM_ALIGN, self.num_align.as_ref());
        let denom_align = calculator.calculate(
            &PROP_DENOM_ALIGN, self.denom_align.as_ref());
        let bevelled = calculator.calculate(
            &PROP_BEVELLED, self.bevelled.as_ref());

        let mut fork = calculator.make_fork();
        if presentation_layout.display_style {
            fork.display_style(false);
        } else {
            fork.script_level(presentation_layout.script_level.new_level(
                PropertyModifier::Increment(1), context, &MathSize::NORMAL,
                presentation_layout.script_size_multiplier, presentation_layout.script_min_size));
        }
        let fork = fork.copy();

        let new_family = family.add(self);

        Box::new(MfracLayout {
            numerator: self.numerator.layout(context, &new_family, &fork, style),
            denominator: self.denominator.layout(context, &new_family, &fork, style),
            dir,
            line_thickness,
            num_align,
            denom_align,
            bevelled,
            presentation_element: presentation_layout,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Mfrac)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mfrac> for Mfrac {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mfrac> for Mfrac {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(
            &Mfrac::new(
                Box::new(Mi::new(String::from("x"))),
                Box::new(Mi::new(String::from("y")))),
            "mfrac");

        let mut mrow = Mrow::new();

        mrow
            .with_child(Box::new(Mi::new(String::from("x"))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mi::new(String::from("y"))));

        let mut mfrac = Mfrac::new(
            Box::new(Mi::new(String::from("a"))),
            Box::new(mrow));

        snap.snap_element(
            &mfrac,
            "mfrac_num_center");

        mfrac.with_num_align(Some(HAlign::Left));
        snap.snap_element(
            &mfrac,
            "mfrac_num_left");

        mfrac.with_num_align(Some(HAlign::Right));
        snap.snap_element(
            &mfrac,
            "mfrac_num_right");

        let mut mrow = Mrow::new();

        mrow
            .with_child(Box::new(Mi::new(String::from("x"))))
            .with_child(Box::new(Mo::new(String::from("+"))))
            .with_child(Box::new(Mi::new(String::from("y"))));

        let mut mfrac = Mfrac::new(
            Box::new(mrow),
            Box::new(Mi::new(String::from("a"))));

        snap.snap_element(
            &mfrac,
            "mfrac_denom_center");

        mfrac.with_denom_align(Some(HAlign::Left));
        snap.snap_element(
            &mfrac,
            "mfrac_denom_left");

        mfrac.with_denom_align(Some(HAlign::Right));
        snap.snap_element(
            &mfrac,
            "mfrac_denom_right");


        mfrac.with_denom_align(Some(HAlign::Center))
            .with_line_thickness(Some(LineThickness::THICK));

        snap.snap_element(
            &mfrac,
            "mfrac_thick_ruler");

        mfrac.with_line_thickness(Some(LineThickness::THIN));

        snap.snap_element(
            &mfrac,
            "mfrac_thin_ruler");

        mfrac.with_bevelled(Some(true));
        snap.snap_element(
            &mfrac,
            "mfrac_bevelled");
    }

    #[test]
    fn it_works_nested() {
        let snap = Snapshot::default();



        let create_nested = | v: &str, x: Box<Element> | {
            let mut nested = Mrow::new();

            nested.with_child(Box::new(Mi::new(String::from(v))))
                .with_child(Box::new(Mo::new(String::from("+"))))
                .with_child(Box::new(Mfrac::new(
                    Box::new(Mn::new(String::from("1"))),
                    x
                )));

            nested
        };

        let nested = create_nested(
            "a", Box::new(create_nested("b", Box::new(
                create_nested("c", Box::new(
                    create_nested("d", Box::new(
                        create_nested("e", Box::new(Mi::new(String::from("f"))))))))))));

        snap.snap_element(&nested, "mfrac_nested");
    }

}