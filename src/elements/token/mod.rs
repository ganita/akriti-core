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


mod mi;                         pub use self::mi::*;

use std::rc::Rc;

use ::props::{MathVariant, MathSize, Directionality};
use ::elements::{Element, PresentationPrivate, Property, InheritedProps, StyleProps};
use ::layout::{TokenLayout};
use ::platform::Context;

pub struct SpecifiedTokenProps {
    pub(crate) text: Rc<String>,

    pub(crate) math_variant: Option<MathVariant>,
    pub(crate) math_size: Option<MathSize>,
    pub(crate) dir: Option<Directionality>,
}


pub trait TokenPrivate<T: Element> : PresentationPrivate<T> {
    const PROP_MATH_VARIANT: Property<MathVariant, T> = Property::Specified {
        default:    || MathVariant::Normal,
        reader:     |s| s.math_variant()
    };

    const PROP_MATH_SIZE: Property<MathSize, T> = Property::Inherited {
        reader:     |i| i.math_size()
    };

    const PROP_DIR: Property<Directionality, T> = Property::Inherited {
        reader:     |i| i.dir()
    };

    fn get_specified_token_props(&self) -> &SpecifiedTokenProps;
    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps;

    fn layout_token_element(
        &self, element: &T, context: &Context, parent: Option<&Element>, inherited: &InheritedProps,
        style: &Option<&StyleProps>) -> TokenLayout {
        let token_props = self.get_specified_token_props();

        let presentation_layout =
            self.layout_presentation(element, context, parent, inherited, style);

        TokenLayout {
            text: token_props.text.clone(),
            math_variant: Self::PROP_MATH_VARIANT.calculate(
                context, element, token_props.math_variant.as_ref(), &parent, inherited, style),
            math_size: presentation_layout.script_level.get_font_size(
                context,
                &Self::PROP_MATH_SIZE.calculate(
                    context, element, token_props.math_size.as_ref(), &parent, inherited, style
                )
            ),
            dir: Self::PROP_DIR.calculate(
                context, element, token_props.dir.as_ref(), &parent, inherited, style),
            presentation_element: presentation_layout,
        }
    }
}

pub trait Token<T: Element> : TokenPrivate<T> {
    fn with_text<'a>(&'a mut self, text: String) -> &'a mut Self {
        self.get_specified_token_props_mut().text = Rc::new(text);
        self
    }

    fn get_text(&self) -> &str {
        &self.get_specified_token_props().text
    }

    fn with_math_variant<'a>(&'a mut self, variant: Option<MathVariant>) -> &'a mut Self {
        self.get_specified_token_props_mut().math_variant = variant;
        self
    }

    fn get_math_variant(&self) -> Option<&MathVariant> {
        self.get_specified_token_props().math_variant.as_ref()
    }

    fn with_math_size<'a>(&'a mut self, size: Option<MathSize>) -> &'a mut Self {
        self.get_specified_token_props_mut().math_size = size;
        self
    }

    fn get_math_size(&self) -> Option<&MathSize> {
        self.get_specified_token_props().math_size.as_ref()
    }

    fn with_dir<'a>(&'a mut self, dir: Option<Directionality>) -> &'a mut Self {
        self.get_specified_token_props_mut().dir = dir;
        self
    }

    fn get_dir(&self) -> Option<&Directionality> {
        self.get_specified_token_props().dir.as_ref()
    }
}