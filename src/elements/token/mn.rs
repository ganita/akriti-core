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

use ::layout::{MnLayout, Layout};
use super::super::{
    TokenPrivate, Token, PresentationPrivate, Presentation, SpecifiedTokenProps, PropertyCalculator,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType, TokenElement};
use ::platform::Context;

pub struct Mn {
    token_props: SpecifiedTokenProps,
    presentation_props: SpecifiedPresentationProps,
}

impl Mn {
    pub fn new(text: String) -> Mn {
        Mn {
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

impl Element for Mn {
    fn layout(&self, context: &Context, parent: Option<&Element>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, parent, inherited, style.clone());

        Box::new(MnLayout {
            token_element: self.layout_token_element(context, &mut calculator)
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mn)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl PresentationPrivate<Mn> for Mn {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl TokenPrivate<Mn> for Mn {
    fn get_specified_token_props(&self) -> &SpecifiedTokenProps {
        &self.token_props
    }

    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps {
        &mut self.token_props
    }
}

impl Token<Mn> for Mn {}

impl Presentation<Mn> for Mn {}


#[cfg(test)]
mod test {
    use super::*;
    use ::test::skia::Snapshot;
    use ::props::{Color, MathSize, MathVariant};

    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        snap.snap_element(
            &Mn::new(String::from("1")),
            "mn_number"
        );

        snap.snap_element(
            Mn::new(String::from("1")).with_math_variant(Some(MathVariant::DoubleStruck)),
            "mn_number_doublestruck"
        );

        snap.snap_element(
            Mn::new(String::from("2")).with_math_size(Some(MathSize::BIG)),
            "mn_big"
        );

        snap.snap_element(
            Mn::new(String::from("34")).with_math_color(Some(Color::RGB(255, 0, 0))),
            "mn_red"
        );

        snap.snap_element(
            Mn::new(String::from("x")).with_math_background(Some(Color::RGB(255, 0, 0))),
            "mn_red_bg"
        );
    }
}