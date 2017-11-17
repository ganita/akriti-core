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

use super::super::{Element, InstanceId, PresentationPrivate, Presentation, SpecifiedPresentationProps,
                 InheritedProps, StyleProps, ElementType, GeneralLayout, PropertyCalculator, Family};
use ::platform::Context;
use ::layout::{Layout, MerrorLayout};

pub struct Merror {
    child: Box<Element>,
    presentation_props: SpecifiedPresentationProps,

    instance_id: InstanceId,
}

impl Merror {
    pub fn new(child: Box<Element>) -> Merror {
        Merror {
            child,
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }
}

impl Element for Merror {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let child_layout = self.child.layout(
            context, &family.add(self), &calculator.make_fork().copy(), style);

        Box::new(MerrorLayout { presentation_layout, child_layout })
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Merror)
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

impl PresentationPrivate<Merror> for Merror {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Merror> for Merror {}

#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::props::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snapshot = Snapshot::default();

        let mut merror = Merror::new(Box::new(Mn::new(String::from("1"))));
        snapshot.snap_element(&merror, "merror_normal");

        merror.with_math_background(Some(Color::RGB(255, 255, 255)));
        merror.with_math_color(Some(Color::RGB(0, 255, 255)));
        snapshot.snap_element(&merror, "merror_white_bg_math_color_sky_blue");
    }
}
