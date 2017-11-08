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

use ::props::*;
use ::layout::{MiLayout, Layout};
use super::super::{
    TokenPrivate, Token, PresentationPrivate, Presentation, SpecifiedTokenProps, PropertyCalculator,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType, TokenElement, Property};
use ::platform::*;

pub struct Mi {
    token_props: SpecifiedTokenProps,
    presentation_props: SpecifiedPresentationProps,
}

impl Mi {
    pub fn new(text: String) -> Mi {
        Mi {
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

impl Element for Mi {
    fn layout(&self, context: &Context, parent: Option<&Element>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, parent, inherited, style.clone());

        Box::new(MiLayout {
            token_element: self.layout_token_element(context, &mut calculator)
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mi)
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl PresentationPrivate<Mi> for Mi {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl TokenPrivate<Mi> for Mi {
    #[allow(const_err)]
    const PROP_MATH_VARIANT: Property<MathVariant, Mi> = Property::Computed {
        default:    || MathVariant::Normal,
        computer:   |_, elm, _| {
            let text = elm.get_text();
            if text.len() == 1 {
                return Some(MathVariant::Italic);
            }
            return None;
        },
        reader:     |s| s.math_variant(),
    };

    fn get_specified_token_props(&self) -> &SpecifiedTokenProps {
        &self.token_props
    }

    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps {
        &mut self.token_props
    }
}

impl Token<Mi> for Mi {}

impl Presentation<Mi> for Mi {}


#[cfg(test)]
mod test {
    use super::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snap = Snapshot::default();
        snap.snap_element(&Mi::new(String::from("i")), "mi_normal");

        snap.snap_element(
            &Mi::new(String::from("i")),
            "mi_normal_identifier"
        );

        snap.snap_element(
            &Mi::new(String::from("ix")),
            "mi_text"
        );

        snap.snap_element(
            Mi::new(String::from("i")).with_math_size(Some(MathSize::BIG)),
            "mi_big"
        );

        snap.snap_element(
            Mi::new(String::from("i")).with_math_color(Some(Color::RGB(255, 0, 0))),
            "mi_red"
        );

        snap.snap_element(
            Mi::new(String::from("i")).with_math_background(Some(Color::RGB(255, 0, 0))),
            "mi_red_bg"
        );
    }
}