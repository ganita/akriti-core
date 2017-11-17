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

use super::super::{Element, InstanceId, Family, InheritedProps, StyleProps, ElementType, GeneralLayout,
                   PropertyCalculator, PresentationPrivate, Presentation, SpecifiedPresentationProps,
                   EmptyComputeCtx, Property};
use ::layout::{Layout, MpaddedLayout};
use ::props::{PseudoLength, PropertyModifier};
use ::platform::Context;

#[allow(const_err)]
const PROP_WIDTH: Property<PropertyModifier<PseudoLength>, Mpadded, EmptyComputeCtx> = Property::Specified {
    default: || PropertyModifier::NoChange,
    reader: |i| i.mpadded_width(),
};

#[allow(const_err)]
const PROP_HEIGHT: Property<PropertyModifier<PseudoLength>, Mpadded, EmptyComputeCtx> = Property::Specified {
    default: || PropertyModifier::NoChange,
    reader: |i| i.mpadded_height(),
};

#[allow(const_err)]
const PROP_DEPTH: Property<PropertyModifier<PseudoLength>, Mpadded, EmptyComputeCtx> = Property::Specified {
    default: || PropertyModifier::NoChange,
    reader: |i| i.mpadded_depth(),
};

#[allow(const_err)]
const PROP_LSPACE: Property<PropertyModifier<PseudoLength>, Mpadded, EmptyComputeCtx> = Property::Specified {
    default: || PropertyModifier::Set(PseudoLength::EM(0.)),
    reader: |i| i.mpadded_lspace(),
};

#[allow(const_err)]
const PROP_VOFFSET: Property<PropertyModifier<PseudoLength>, Mpadded, EmptyComputeCtx> = Property::Specified {
    default: || PropertyModifier::Set(PseudoLength::EM(0.)),
    reader: |i| i.mpadded_voffset(),
};


pub struct Mpadded {
    child: Box<Element>,

    width: Option<PropertyModifier<PseudoLength>>,
    height: Option<PropertyModifier<PseudoLength>>,
    depth: Option<PropertyModifier<PseudoLength>>,
    lspace: Option<PropertyModifier<PseudoLength>>,
    voffset: Option<PropertyModifier<PseudoLength>>,

    presentation_props: SpecifiedPresentationProps,

    instance_id: InstanceId,
}

impl Mpadded {
    pub fn new(child: Box<Element>) -> Mpadded {
        Mpadded {
            child,
            width: None,
            height: None,
            depth: None,
            lspace: None,
            voffset: None,
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }
    
    pub fn with_width<'a>(&'a mut self, width: Option<PropertyModifier<PseudoLength>>) -> &'a mut Mpadded {
        self.width = width;
        self
    }
    
    pub fn width(&self) -> Option<&PropertyModifier<PseudoLength>> {
        self.width.as_ref()
    }

    pub fn with_height<'a>(&'a mut self, height: Option<PropertyModifier<PseudoLength>>) -> &'a mut Mpadded {
        self.height = height;
        self
    }

    pub fn height(&self) -> Option<&PropertyModifier<PseudoLength>> {
        self.height.as_ref()
    }

    pub fn with_depth<'a>(&'a mut self, depth: Option<PropertyModifier<PseudoLength>>) -> &'a mut Mpadded {
        self.depth = depth;
        self
    }

    pub fn depth(&self) -> Option<&PropertyModifier<PseudoLength>> {
        self.depth.as_ref()
    }

    pub fn with_lspace<'a>(&'a mut self, lspace: Option<PropertyModifier<PseudoLength>>) -> &'a mut Mpadded {
        self.lspace = lspace;
        self
    }

    pub fn lspace(&self) -> Option<&PropertyModifier<PseudoLength>> {
        self.lspace.as_ref()
    }

    pub fn with_voffset<'a>(&'a mut self, voffset: Option<PropertyModifier<PseudoLength>>) -> &'a mut Mpadded {
        self.voffset = voffset;
        self
    }

    pub fn voffset(&self) -> Option<&PropertyModifier<PseudoLength>> {
        self.voffset.as_ref()
    }
}

impl Element for Mpadded {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let width = calculator.calculate(
            &PROP_WIDTH, self.width.as_ref());
        let height = calculator.calculate(
            &PROP_HEIGHT, self.height.as_ref());
        let depth = calculator.calculate(
            &PROP_DEPTH, self.depth.as_ref());
        let lspace = calculator.calculate(
            &PROP_LSPACE, self.lspace.as_ref());
        let voffset = calculator.calculate(
            &PROP_VOFFSET, self.voffset.as_ref());

        let presentation_layout = self.layout_presentation(&mut calculator);

        let inherited_fork = calculator.make_fork().copy();
        let new_family = family.add(self);

        Box::new(MpaddedLayout {
            width,
            height,
            depth,
            lspace,
            voffset,
            child_layout: self.child.layout(context, &new_family, &inherited_fork, style),
            presentation_layout,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Mpadded)
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

impl PresentationPrivate<Mpadded> for Mpadded {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mpadded> for Mpadded {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;
    use ::props::*;

    #[test]
    fn it_works() {
        let mut mrow = Mrow::new();
        mrow.with_child(Box::new(Mi::new(String::from("x"))));
        let mpadded = Mpadded::new(Box::new(Mi::new(String::from("y"))));

        mrow.with_child(Box::new(mpadded));
        mrow.with_child(Box::new(Mi::new(String::from("z"))));

        Snapshot::default().snap_element(&mrow, "mpadded_no_change");

        let mut mrow = Mrow::new();
        mrow.with_child(Box::new(Mi::new(String::from("x"))));
        let mut mpadded = Mpadded::new(Box::new(Mi::new(String::from("y"))));
        mpadded.with_lspace(Some(PropertyModifier::Increment(PseudoLength::DU(5f32))));

        mrow.with_child(Box::new(mpadded));
        mrow.with_child(Box::new(Mi::new(String::from("z"))));

        Snapshot::default().snap_element(&mrow, "mpadded_lspace");

        let mut mrow = Mrow::new();
        mrow.with_child(Box::new(Mi::new(String::from("x"))));
        let mut mpadded = Mpadded::new(Box::new(Mi::new(String::from("y"))));
        mpadded.with_voffset(Some(PropertyModifier::Decrement(PseudoLength::DU(5f32))));

        mrow.with_child(Box::new(mpadded));
        mrow.with_child(Box::new(Mi::new(String::from("z"))));

        Snapshot::default().snap_element(&mrow, "mpadded_voffset");

        let mut mrow = Mrow::new();
        mrow.with_child(Box::new(Mi::new(String::from("x"))));
        let mut mpadded = Mpadded::new(Box::new(Mi::new(String::from("y"))));
        mpadded.with_width(Some(PropertyModifier::Set(PseudoLength::Width(0.85f32))));

        mrow.with_child(Box::new(mpadded));
        mrow.with_child(Box::new(Mi::new(String::from("z"))));
        Snapshot::default().snap_element(&mrow, "mpadded_width");

        let mut mrow = Mrow::new();
        mrow.with_child(Box::new(Mi::new(String::from("x"))));
        let mut mpadded = Mpadded::new(Box::new(Mi::new(String::from("y"))));
        mpadded.with_math_color(Some(Color::RGB(0, 0, 255)));
        mpadded.with_math_background(Some(Color::RGB(255, 255, 255)));

        mrow.with_child(Box::new(mpadded));
        mrow.with_child(Box::new(Mi::new(String::from("z"))));
        Snapshot::default().snap_element(&mrow, "mpadded_math_color_blue_bg_white");
    }
}