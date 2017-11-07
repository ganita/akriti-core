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
    TokenPrivate, Token, PresentationPrivate, Presentation, SpecifiedTokenProps,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType, TokenElement};
use ::draw::*;
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
        Box::new(self.layout_token_element(self, context, parent, inherited, style))
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
    fn get_specified_token_props(&self) -> &SpecifiedTokenProps {
        &self.token_props
    }

    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps {
        &mut self.token_props
    }
}

impl Token<Mi> for Mi {}

impl Presentation<Mi> for Mi {}