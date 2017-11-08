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


use std::rc::Rc;
use std::any::Any;

use ::layout::{MtextLayout, Layout};
use super::super::{
    TokenPrivate, Token, PresentationPrivate, Presentation, SpecifiedTokenProps, PropertyCalculator,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType,
    TokenElement, InstanceId, Family};
use ::platform::Context;

pub struct Mtext {
    instance_id: InstanceId,
    token_props: SpecifiedTokenProps,
    presentation_props: SpecifiedPresentationProps,
}

impl Mtext {
    pub fn new(text: String) -> Mtext {
        Mtext {
            instance_id: InstanceId::new(),
            token_props: SpecifiedTokenProps {
                text: Rc::new(text),
                math_variant: None,
                math_size: None,
                dir: None,
            },
            presentation_props: SpecifiedPresentationProps {
                math_color: None,
                math_background: None,
            }
        }
    }
}

impl Element for Mtext {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        Box::new(MtextLayout {
            token_element: self.layout_token_element(context, &mut calculator)
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mtext)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mtext> for Mtext {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl TokenPrivate<Mtext> for Mtext {
    fn get_specified_token_props(&self) -> &SpecifiedTokenProps {
        &self.token_props
    }

    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps {
        &mut self.token_props
    }
}

impl Token<Mtext> for Mtext {}

impl Presentation<Mtext> for Mtext {}


#[cfg(test)]
mod test {
    use super::*;
    use ::test::skia::Snapshot;
    use ::props::{Color, MathSize, MathVariant};

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(
            &Mtext::new(String::from("hello world!")),
            "mtext_normal"
        );

        snap.snap_element(
            Mtext::new(String::from("double struck text")).with_math_variant(Some(MathVariant::DoubleStruck)),
            "mtext_variant"
        );

        snap.snap_element(
            Mtext::new(String::from("this is big")).with_math_size(Some(MathSize::BIG)),
            "mtext_big"
        );

        snap.snap_element(
            Mtext::new(String::from("this is written in red")).with_math_color(Some(Color::RGB(255, 0, 0))),
            "mtext_red"
        );

        snap.snap_element(
            Mtext::new(String::from("this is written on red bg")).with_math_background(Some(Color::RGB(255, 0, 0))),
            "mtext_red_bg"
        );
    }
}