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

use ::layout::{MsLayout, Layout};
use super::super::{
    TokenPrivate, Token, PresentationPrivate, Presentation, SpecifiedTokenProps, PropertyCalculator,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType, TokenElement,
    Property, InstanceId, Family, DefaultComputationContext};
use ::platform::Context;

const PROP_LQUOTE: Property<String, Ms, DefaultComputationContext> = Property::Specified {
    default: || String::from("\""),
    reader: |s| s.ms_lquote(),
};

const PROP_RQUOTE: Property<String, Ms, DefaultComputationContext> = Property::Specified {
    default: || String::from("\""),
    reader: |s| s.ms_rquote(),
};

pub struct Ms {
    instance_id: InstanceId,
    lquote: Option<String>,
    rquote: Option<String>,

    token_props: SpecifiedTokenProps,
    presentation_props: SpecifiedPresentationProps,
}

impl Ms {
    pub fn new(text: String) -> Ms {
        Ms {
            instance_id: InstanceId::new(),
            lquote: None,
            rquote: None,
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

impl Element for Ms {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {

        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());


        let lquote = calculator.calculate(&PROP_LQUOTE, self.lquote.as_ref());
        let rquote = calculator.calculate(&PROP_RQUOTE, self.rquote.as_ref());

        let token_layout = MsLayout {
            token_element: self.layout_token_element(context, &mut calculator),
            rquote,
            lquote
        };

        Box::new(token_layout)
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Ms)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Ms> for Ms {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl TokenPrivate<Ms> for Ms {
    fn get_specified_token_props(&self) -> &SpecifiedTokenProps {
        &self.token_props
    }

    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps {
        &mut self.token_props
    }
}

impl Token<Ms> for Ms {}

impl Presentation<Ms> for Ms {}


#[cfg(test)]
mod test {
    use super::*;
    use ::test::skia::Snapshot;
    use ::props::{Color, MathSize, MathVariant};

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(
            &Ms::new(String::from("hello ms")),
            "ms_text"
        );

        snap.snap_element(
            Ms::new(String::from("hello fraktur"))
                .with_math_variant(Some(MathVariant::Fraktur)),
            "ms_text_fraktur"
        );

        snap.snap_element(
            Ms::new(String::from("I am big"))
                .with_math_size(Some(MathSize::BIG)),
            "ms_big"
        );

        snap.snap_element(
            Ms::new(String::from("I am red"))
                .with_math_color(Some(Color::RGB(255, 0, 0))),
            "ms_red"
        );

        snap.snap_element(
            Ms::new(String::from("I am on fire"))
                .with_math_background(Some(Color::RGB(255, 0, 0))),
            "ms_red_bg"
        );
    }
}