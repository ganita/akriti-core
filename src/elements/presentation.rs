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


use ::props::*;
use ::layout::PresentationLayout;
use super::{Property, Element, InheritedProps, StyleProps};
use ::platform::Context;

#[derive(Default)]
pub struct SpecifiedPresentationProps {
    pub(crate) math_color: Option<Color>,
    pub(crate) math_background: Option<Color>,
}

pub trait PresentationPrivate<T: Element> {
    const PROP_MATH_COLOR: Property<Color, T> = Property::Inherited {
        reader:                 |p| p.math_color(),
    };

    const PROP_MATH_BACKGROUND: Property<Color, T> = Property::Specified {
        default:                || Color::transparent(),
        reader:                 |s| s.math_background(),
    };

    const PROP_DISPLAY_STYLE: Property<DisplayStyle, T> = Property::Inherited {
        reader:                 |p| p.display_style(),
    };

    const PROP_SCRIPT_LEVEL: Property<ScriptLevel, T> = Property::Inherited {
        reader:                 |p| p.script_level(),
    };

    const PROP_SCRIPT_MIN_SIZE: Property<ScriptMinSize, T> = Property::Inherited {
        reader:                 |p| p.script_min_size(),
    };

    const PROP_SCRIPT_SIZE_MULTIPLIER: Property<ScriptSizeMultiplier, T> = Property::Inherited {
        reader:                 |p| p.script_size_multiplier(),
    };

    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps;
    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps;

    fn layout_presentation(
        &self, element: &T, context: &Context, parent: Option<&Element>, inherited: &InheritedProps,
        style: &Option<&StyleProps>) -> PresentationLayout {
        let specified = self.get_specified_presentation_props();

        PresentationLayout {
            math_color: Self::PROP_MATH_COLOR.calculate(
                context, element, specified.math_color.as_ref(), &parent, inherited, style),
            math_background: Self::PROP_MATH_BACKGROUND.calculate(
                context, element, specified.math_background.as_ref(), &parent, inherited, style),
            display_style: Self::PROP_DISPLAY_STYLE.calculate(
                context, element, None, &parent, inherited, style),
            script_level: Self::PROP_SCRIPT_LEVEL.calculate(
                context, element, None, &parent, inherited, style),
            script_min_size: Self::PROP_SCRIPT_MIN_SIZE.calculate(
                context, element, None, &parent, inherited, style),
            script_size_multiplier: Self::PROP_SCRIPT_SIZE_MULTIPLIER.calculate(
                context, element, None, &parent, inherited, style),
        }
    }
}

pub trait Presentation<T: Element> : PresentationPrivate<T> + Sized {
    fn with_math_color<'a>(&'a mut self, color: Option<Color>) -> &'a mut Self {
        self.get_specified_presentation_props_mut().math_color = color;
        self
    }

    fn with_math_background<'a>(&'a mut self, color: Option<Color>) -> &'a Self {
        self.get_specified_presentation_props_mut().math_background = color;
        self
    }

    fn get_math_color(&self) -> Option<&Color> {
        self.get_specified_presentation_props().math_color.as_ref()
    }

    fn get_math_background(&self) -> Option<&Color> {
        self.get_specified_presentation_props().math_background.as_ref()
    }
}