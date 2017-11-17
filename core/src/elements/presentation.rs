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
use super::{Property, Element, PropertyCalculator, EmptyComputeCtx};

#[derive(Default)]
pub struct SpecifiedPresentationProps {
    pub(crate) math_color: Option<Color>,
    pub(crate) math_background: Option<Color>,
}

pub trait PresentationPrivate<T: Element> {
    #[allow(const_err)]
    const PROP_MATH_COLOR: Property<Color, T, EmptyComputeCtx> = Property::Inherited {
        reader:                 |p| p.math_color(),
        writer:                 |v, fork| fork.math_color(v)
    };

    #[allow(const_err)]
    const PROP_MATH_BACKGROUND: Property<Color, T, EmptyComputeCtx> = Property::Specified {
        default:                || Color::transparent(),
        reader:                 |s| s.math_background(),
    };

    #[allow(const_err)]
    const PROP_DISPLAY_STYLE: Property<DisplayStyle, T, EmptyComputeCtx> = Property::Inherited {
        reader:                 |p| p.display_style(),
        writer:                 |v, fork| fork.display_style(v)
    };

    #[allow(const_err)]
    const PROP_SCRIPT_LEVEL: Property<ScriptLevel, T, EmptyComputeCtx> = Property::Inherited {
        reader:                 |p| p.script_level(),
        writer:                 |v, fork| fork.script_level(v)
    };

    #[allow(const_err)]
    const PROP_SCRIPT_MIN_SIZE: Property<ScriptMinSize, T, EmptyComputeCtx> = Property::Inherited {
        reader:                 |p| p.script_min_size(),
        writer:                 |v, fork| fork.script_min_size(v)
    };

    #[allow(const_err)]
    const PROP_SCRIPT_SIZE_MULTIPLIER: Property<ScriptSizeMultiplier, T, EmptyComputeCtx> = Property::Inherited {
        reader:                 |p| p.script_size_multiplier(),
        writer:                 |v, fork| fork.script_size_multiplier(v)
    };

    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps;
    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps;

    fn layout_presentation(&self, calculator: &mut PropertyCalculator<T>) -> PresentationLayout {
        let specified = self.get_specified_presentation_props();

        PresentationLayout {
            math_color: calculator.calculate(
                &Self::PROP_MATH_COLOR, specified.math_color.as_ref()),
            math_background: calculator.calculate(
                &Self::PROP_MATH_BACKGROUND, specified.math_background.as_ref()),
            display_style: calculator.calculate(&Self::PROP_DISPLAY_STYLE, None),
            script_level: calculator.calculate(&Self::PROP_SCRIPT_LEVEL, None),
            script_min_size: calculator.calculate(&Self::PROP_SCRIPT_MIN_SIZE, None),
            script_size_multiplier: calculator.calculate(&Self::PROP_SCRIPT_SIZE_MULTIPLIER, None),
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