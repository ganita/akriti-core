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

use super::super::{Element, InheritedProps, StyleProps, Family, ElementType, InstanceId,
                   SpecifiedPresentationProps, PresentationPrivate, Presentation, ScriptsAndLimits,
                   PropertyCalculator, Property, EmptyComputeCtx};
use ::layout::{Layout, MunderoverLayout, MmultiscriptLayout};
use ::platform::Context;
use ::props::{HAlign, PropertyModifier, MathSize, Accent};
use ::utils::get_core_mo_layout;

#[allow(const_err)]
const PROP_ACCENT: Property<Accent, Munderover, EmptyComputeCtx> = Property::Specified {
    default: || Accent::Automatic,
    reader: |i| i.underover_accent(),
};

#[allow(const_err)]
const PROP_ACCENT_UNDER: Property<Accent, Munderover, EmptyComputeCtx> = Property::Specified {
    default: || Accent::Automatic,
    reader: |i| i.underover_accent_under(),
};

#[allow(const_err)]
const PROP_ALIGN: Property<HAlign, Munderover, EmptyComputeCtx> = Property::Specified {
    default: || HAlign::Center,
    reader: |i| i.underover_align(),
};

pub struct Munderover {
    base: Box<Element>,
    overscript: Box<Element>,
    underscript: Box<Element>,

    accent: Option<Accent>,
    accent_under: Option<Accent>,
    align: Option<HAlign>,

    presentation_props: SpecifiedPresentationProps,

    instance_id: InstanceId,
}

impl Munderover {
    pub fn new(base: Box<Element>, overscript: Box<Element>, underscript: Box<Element>) -> Munderover {
        Munderover {
            base,
            overscript,
            underscript,
            accent: None,
            accent_under: None,
            align: None,
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub fn with_base<'a>(&'a mut self, base: Box<Element>) -> &'a mut Munderover {
        self.base = base;
        self
    }

    pub fn base(&self) -> &Box<Element> {
        &self.base
    }

    pub fn with_overscript<'a>(&'a mut self, overscript: Box<Element>) -> &'a mut Munderover {
        self.overscript = overscript;
        self
    }

    pub fn overscript(&self) -> &Box<Element> {
        &self.overscript
    }

    pub fn with_underscript<'a>(&'a mut self, underscript: Box<Element>) -> &'a mut Munderover {
        self.underscript = underscript;
        self
    }

    pub fn underscript(&self) -> &Box<Element> {
        &self.underscript
    }

    pub fn with_accent<'a>(&'a mut self, accent: Option<Accent>) -> &'a mut Munderover {
        self.accent = accent;
        self
    }

    pub fn accent(&self) -> Option<&Accent> {
        self.accent.as_ref()
    }

    pub fn with_accent_under<'a>(&'a mut self, accent_under: Option<Accent>) -> &'a mut Munderover {
        self.accent_under = accent_under;
        self
    }

    pub fn accent_under(&self) -> Option<&Accent> {
        self.accent_under.as_ref()
    }

    pub fn with_align<'a>(&'a mut self, align: Option<HAlign>) -> &'a mut Munderover {
        self.align = align;
        self
    }

    pub fn align(&self) -> Option<&HAlign> {
        self.align.as_ref()
    }
}

impl Element for Munderover {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);

        let align = calculator.calculate(
            &PROP_ALIGN, self.align.as_ref());
        let accent = calculator.calculate(
            &PROP_ACCENT, self.accent.as_ref());
        let accent_under = calculator.calculate(
            &PROP_ACCENT_UNDER, self.accent.as_ref());

        let new_family = family.add(self);
        let inherited_base = calculator.make_fork().copy();

        let base_layout = self.base.layout(context, &new_family, &inherited_base, style);

        let (mo_movable_limits, mo_accent) = {
            let core_mo = get_core_mo_layout(&base_layout);
            if let Some(mo) = core_mo {
                (mo.movable_limits, mo.accent)
            } else {
                (false, false)
            }
        };

        if !presentation_layout.display_style && mo_movable_limits {
            let mut inherited_scripts = inherited_base.copier();
            inherited_scripts.display_style(false);
            inherited_scripts.script_level(presentation_layout.script_level.new_level(
                PropertyModifier::Increment(1), context, &MathSize::NORMAL,
                presentation_layout.script_size_multiplier, presentation_layout.script_min_size
            ));
            let inherited_scripts = inherited_scripts.copy();

            return Box::new(MmultiscriptLayout {
                base_layout,
                prescript_layout: Vec::new(),
                postscript_layout: vec![(
                    self.overscript.layout(context, &new_family, &inherited_scripts, style),
                    self.underscript.layout(context, &new_family, &inherited_scripts, style)
                )],
                subscript_shift: 0.0,
                superscript_shift: 0.0,
                presentation_layout,
            });
        }


        let accent = accent.get_accent(mo_accent);
        let accent_under = accent_under.get_accent(mo_accent);

        let mut inherited_over = inherited_base.copier();
        inherited_over.display_style(false);
        if !accent {
            inherited_over.script_level(presentation_layout.script_level.new_level(
                PropertyModifier::Increment(1), context, &MathSize::NORMAL,
                presentation_layout.script_size_multiplier, presentation_layout.script_min_size
            ));
        }
        let inherited_over = inherited_over.copy();

        let mut inherited_under = inherited_base.copier();
        inherited_under.display_style(false);
        if !accent_under {
            inherited_under.script_level(presentation_layout.script_level.new_level(
                PropertyModifier::Increment(1), context, &MathSize::NORMAL,
                presentation_layout.script_size_multiplier, presentation_layout.script_min_size
            ));
        }
        let inherited_under = inherited_under.copy();

        return Box::new(MunderoverLayout {
            accent_over: accent,
            accent_under,
            align,
            base: base_layout,
            underscript: self.underscript.layout(context, &new_family, &inherited_under, style),
            overscript: self.overscript.layout(context, &new_family, &inherited_over, style),
            presentation_layout,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::ScriptsAndLimits(ScriptsAndLimits::Munderover)
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

impl PresentationPrivate<Munderover> for Munderover {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Munderover> for Munderover {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;
    
    #[test]
    fn it_works() {
        let snapshot = Snapshot::default();
        let underover = Munderover::new(
            Box::new(Mi::new(String::from("x"))),
            Box::new(Mn::new(String::from("x"))),
            Box::new(Mn::new(String::from("x"))),
        );

        snapshot.snap_element(&underover, "munderover_simple");
    }
}