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

use ::layout::{MspaceLayout, Layout};
use super::super::{
    PresentationPrivate, Presentation, PropertyCalculator, Property,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType,
    TokenElement, InstanceId, Family, DefaultComputationContext};
use ::platform::Context;
use ::props::{Length, MathVariant, MathSize, Directionality, LineBreak};

#[allow(const_err)]
const PROP_WIDTH: Property<Length, Mspace, DefaultComputationContext> = Property::Specified {
    default: || Length::EX(0.),
    reader: |s| s.mspace_width(),
};

#[allow(const_err)]
const PROP_HEIGHT: Property<Length, Mspace, DefaultComputationContext> = Property::Specified {
    default: || Length::EX(0.),
    reader: |s| s.mspace_height(),
};

#[allow(const_err)]
const PROP_DEPTH: Property<Length, Mspace, DefaultComputationContext> = Property::Specified {
    default: || Length::EX(0.),
    reader: |s| s.mspace_depth(),
};

#[allow(const_err)]
const PROP_LINEBREAK: Property<LineBreak, Mspace, DefaultComputationContext> = Property::Specified {
    default: || LineBreak::Auto,
    reader: |s| s.linebreak(),
};

#[allow(const_err)]
const PROP_MATH_SIZE: Property<MathSize, Mspace, DefaultComputationContext> = Property::Inherited {
    reader:     |i| i.math_size(),
    writer:     |v, fork| fork.math_size(v)
};

pub struct Mspace {
    instance_id: InstanceId,
    width: Option<Length>,
    height: Option<Length>,
    depth: Option<Length>,
    linebreak: Option<LineBreak>,

    math_variant: Option<MathVariant>,
    math_size: Option<MathSize>,
    dir: Option<Directionality>,

    presentation_props: SpecifiedPresentationProps,
}

impl Mspace {
    pub fn new() -> Mspace {
        Mspace {
            instance_id: InstanceId::new(),
            width: None,
            height: None,
            depth: None,
            linebreak: None,
            math_variant: None,
            math_size: None,
            dir: None,
            presentation_props: SpecifiedPresentationProps {
                math_color: None,
                math_background: None,
            },
        }
    }
    
    pub fn with_width<'a>(&'a mut self, width: Option<Length>) -> &'a mut Mspace {
        self.width = width;
        self
    }
    
    pub fn get_width(&self) -> Option<&Length> {
        self.width.as_ref()
    }

    pub fn with_height<'a>(&'a mut self, height: Option<Length>) -> &'a mut Mspace {
        self.height = height;
        self
    }

    pub fn get_height(&self) -> Option<&Length> {
        self.height.as_ref()
    }

    pub fn with_depth<'a>(&'a mut self, depth: Option<Length>) -> &'a mut Mspace {
        self.depth = depth;
        self
    }

    pub fn get_depth(&self) -> Option<&Length> {
        self.depth.as_ref()
    }

    pub fn with_linebreak<'a>(&'a mut self, linebreak: Option<LineBreak>) -> &'a mut Mspace {
        self.linebreak = linebreak;
        self
    }

    pub fn get_linebreak(&self) -> Option<&LineBreak> {
        self.linebreak.as_ref()
    }

    pub fn with_math_variant<'a>(&'a mut self, math_variant: Option<MathVariant>) -> &'a mut Mspace {
        self.math_variant = math_variant;
        self
    }

    pub fn get_math_variant(&self) -> Option<&MathVariant> {
        self.math_variant.as_ref()
    }

    pub fn with_math_size<'a>(&'a mut self, math_size: Option<MathSize>) -> &'a mut Mspace {
        self.math_size = math_size;
        self
    }

    pub fn get_math_size(&self) -> Option<&MathSize> {
        self.math_size.as_ref()
    }

    pub fn with_dir<'a>(&'a mut self, dir: Option<Directionality>) -> &'a mut Mspace {
        self.dir = dir;
        self
    }

    pub fn get_dir(&self) -> Option<&Directionality> {
        self.dir.as_ref()
    }
}

impl Element for Mspace {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        let mut property_calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut property_calculator);

        let math_size = property_calculator.calculate(
            &PROP_MATH_SIZE, self.math_size.as_ref());

        let current_font_size = presentation_layout.script_level.get_font_size(
            context, &math_size);

        Box::new(MspaceLayout {
            width: property_calculator.calculate(&PROP_WIDTH, self.width.as_ref())
                .get_length_du(context, current_font_size),
            height: property_calculator.calculate(&PROP_HEIGHT, self.height.as_ref())
                .get_length_du(context, current_font_size),
            depth: property_calculator.calculate(&PROP_DEPTH, self.depth.as_ref())
                .get_length_du(context, current_font_size),
            _linebreak: property_calculator.calculate(&PROP_LINEBREAK, self.linebreak.as_ref()),
            presentation_layout,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mspace)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mspace> for Mspace {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mspace> for Mspace {}


#[cfg(test)]
mod test {
    use super::*;
    use ::test::skia::Snapshot;
    use ::props::{Color, MathSize};

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(Mspace::new()
                              .with_depth(Some(Length::EM(1.)))
                              .with_height(Some(Length::EM(1.)))
                              .with_width(Some(Length::EM(1.))),
                          "mspace_normal");

        snap.snap_element(Mspace::new()
                              .with_depth(Some(Length::EM(1.)))
                              .with_height(Some(Length::EM(1.)))
                              .with_width(Some(Length::EM(1.)))
                              .with_math_background(Some(Color::RGB(255, 0, 0))),
                          "mspace_red_bg");

        snap.snap_element(Mspace::new()
                              .with_depth(Some(Length::EM(1.)))
                              .with_height(Some(Length::EM(1.)))
                              .with_width(Some(Length::EM(1.)))
                              .with_math_size(Some(MathSize::BIG)),
                          "mspace_big");
    }
}