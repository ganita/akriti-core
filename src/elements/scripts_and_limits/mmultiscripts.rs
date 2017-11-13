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

use super::super::{Element, ElementType, Family, InheritedProps, StyleProps, InstanceId,
                   ScriptsAndLimits, PropertyCalculator, Presentation, PresentationPrivate,
                   SpecifiedPresentationProps, EmptyComputeCtx, Property};
use ::platform::Context;
use ::layout::{Layout, MmultiscriptLayout};
use ::props::{Length, MathSize, PropertyModifier};

#[allow(const_err)]
const PROP_SUPERSCRIPT_SHIFT: Property<Length, Mmultiscripts, EmptyComputeCtx> = Property::Specified {
    default: || Length::Auto,
    reader: |i| i.superscript_shift(),
};

#[allow(const_err)]
const PROP_SUBSCRIPT_SHIFT: Property<Length, Mmultiscripts, EmptyComputeCtx> = Property::Specified {
    default: || Length::Auto,
    reader: |i| i.subscript_shift(),
};

pub struct Mmultiscript {
    superscript: Box<Element>,
    subscript: Box<Element>,
}

pub struct Mmultiscripts {
    base: Box<Element>,
    prescripts: Vec<Mmultiscript>,
    postscripts: Vec<Mmultiscript>,
    superscript_shift: Option<Length>,
    subscript_shift: Option<Length>,

    presentation_props: SpecifiedPresentationProps,

    instance_id: InstanceId,
}

impl Mmultiscripts {
    pub fn new(base: Box<Element>) -> Mmultiscripts {
        Mmultiscripts {
            base,
            prescripts: Vec::new(),
            postscripts: Vec::new(),
            superscript_shift: None,
            subscript_shift: None,
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub fn with_baset<'a>(&'a mut self, base: Box<Element>) -> &'a mut Mmultiscripts {
        self.base = base;
        self
    }

    pub fn base(&self) -> &Box<Element> {
        &self.base
    }

    pub fn with_prescript<'a>(&'a mut self, prescript: Mmultiscript) -> &'a mut Mmultiscripts {
        self.prescripts.push(prescript);
        self
    }

    pub fn prescript(&self) -> &[Mmultiscript] {
        &self.prescripts[..]
    }

    pub fn with_postscript<'a>(&'a mut self, postscript: Mmultiscript) -> &'a mut Mmultiscripts {
        self.postscripts.push(postscript);
        self
    }

    pub fn postscript(&self) -> &[Mmultiscript] {
        &self.postscripts[..]
    }

    pub fn with_superscript_shift<'a>(&'a mut self, superscript_shift: Option<Length>) -> &'a mut Mmultiscripts {
        self.superscript_shift = superscript_shift;
        self
    }

    pub fn superscript_shift(&self) -> Option<&Length> {
        self.superscript_shift.as_ref()
    }

    pub fn with_subscript_shift<'a>(&'a mut self, subscript_shift: Option<Length>) -> &'a mut Mmultiscripts {
        self.subscript_shift = subscript_shift;
        self
    }

    pub fn subscript_shift(&self) -> Option<&Length> {
        self.subscript_shift.as_ref()
    }
}

impl Element for Mmultiscripts {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);

        let superscript_shift = calculator.calculate(
            &PROP_SUPERSCRIPT_SHIFT, self.superscript_shift.as_ref());
        let subscript_shift = calculator.calculate(
            &PROP_SUBSCRIPT_SHIFT, self.subscript_shift.as_ref());
        let base_size = presentation_layout.script_level.get_font_size(context, &MathSize::NORMAL);

        let new_family = family.add(self);
        let inherited_fork = calculator.make_fork().copy();

        let mut inherited_fork_scripts = inherited_fork.copier();
        inherited_fork_scripts.display_style(false);
        inherited_fork_scripts.script_level(presentation_layout.script_level.new_level(
            PropertyModifier::Increment(1),
            context, &MathSize::NORMAL, presentation_layout.script_size_multiplier,
            presentation_layout.script_min_size
        ));
        let inherited_fork_scripts = inherited_fork_scripts.copy();

        Box::new(MmultiscriptLayout {
            base_layout: self.base.layout(context, &new_family, &inherited_fork, style),
            prescript_layout: self.prescripts.iter().map(| multiscript | {
                (
                    multiscript.superscript.layout(context, &new_family, &inherited_fork_scripts, style),
                    multiscript.subscript.layout(context, &new_family, &inherited_fork_scripts, style),
                )
            }).collect(),
            postscript_layout: self.postscripts.iter().map(| multiscript | {
                (
                    multiscript.superscript.layout(context, &new_family, &inherited_fork_scripts, style),
                    multiscript.subscript.layout(context, &new_family, &inherited_fork_scripts, style),
                )
            }).collect(),
            subscript_shift: subscript_shift.get_length_du(context, base_size),
            superscript_shift: superscript_shift.get_length_du(context, base_size),
            presentation_layout,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::ScriptsAndLimits(ScriptsAndLimits::Mmutliscripts)
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

impl PresentationPrivate<Mmultiscripts> for Mmultiscripts {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mmultiscripts> for Mmultiscripts {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::props::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snapshot = Snapshot::default();

        let mut script = Mmultiscripts::new(Box::new(Mi::new(String::from("x"))));
        snapshot.snap_element(&script, "mmutliscript_base_only");

        script.with_prescript(
            Mmultiscript {
                superscript: Box::new(Mn::new(String::from("1"))),
                subscript: Box::new(Mn::new(String::from("2")))
            }
        );

        snapshot.snap_element(&script, "mmutliscript_prescript_only");

        script.with_postscript(
            Mmultiscript {
                superscript: Box::new(Mn::new(String::from("3"))),
                subscript: Box::new(Mn::new(String::from("4")))
            }
        );

        snapshot.snap_element(&script, "mmutliscript_prescript_postscript");
    }
}