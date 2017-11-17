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
use ::layout::{MoLayout, Layout};
use super::super::{
    TokenPrivate, Token, PresentationPrivate, Presentation, SpecifiedTokenProps, PropertyCalculator,
    SpecifiedPresentationProps, Element, InheritedProps, StyleProps, ElementType, TokenElement, Property,
    Mrow, InstanceId, Family, EmptyComputeCtx};
use ::platform::*;
use ::utils::{is_space_like, get_enclosing_embellished_operator};
use ::constants::SpaceLevel;

#[derive(Debug)]
struct FormRequiredComputationContext {
    form: OperatorForm
}

impl FormRequiredComputationContext {
    pub fn new(form: OperatorForm) -> FormRequiredComputationContext {
        FormRequiredComputationContext { form }
    }

    pub fn get_form(&self) -> &OperatorForm {
        &self.form
    }
}

#[allow(const_err)]
const PROP_FORM: Property<OperatorForm, Mo, EmptyComputeCtx> = Property::Computed {
    default: || OperatorForm::Infix,
    computer: |ctx, elm, family, _| {
        let operator = elm.get_text();
        let op_dict = ctx.operator_dictionary();
        let forms = op_dict.operator_forms(operator);

        if forms.is_none() {
            return None;
        }
        let forms = forms.unwrap();

        if forms.is_empty() {
            return None;
        }

        if forms.len() == 1 {
            let (form, _) = forms.entries().next().unwrap();
            return Some(form.clone());
        }

        let embellished_operator = get_enclosing_embellished_operator(family, elm).unwrap_or(family);
        let parent = embellished_operator.parent();

        if parent.is_none() {
            return None;
        }

        let parent = parent.unwrap();

        if !parent.type_info().is_mrow() {
            return None;
        }

        let parent = parent.as_any().downcast_ref::<Mrow>().unwrap();
        let children = parent.children();

        if children.len() == 1 {
            if let Some(grand_parent_family) = family.grand_parent() {
                if let Some(grand_parent) = grand_parent_family.parent() {
                    if grand_parent.type_info().is_scrips_or_limits() {
                        return Some(OperatorForm::Postfix);
                    }
                }
            }

            return None;
        }

        let num_non_space_like = children.iter()
            .fold(0, |acc, child| if !is_space_like(child.as_ref()) { acc+1 } else { acc });

        if num_non_space_like < 2 {
            return None;
        }

        if children.iter().find(|c| !is_space_like(c.as_ref())).unwrap().instance_id() == elm.instance_id() {
            return Some(OperatorForm::Prefix);
        }

        if children.iter().rev().find(|c| !is_space_like(c.as_ref())).unwrap().instance_id() == elm.instance_id() {
            return Some(OperatorForm::Postfix);
        }

        return None;
    },
    reader: |s| s.mo_form(),
};

#[allow(const_err)]
const _PROP_FENCE: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().fence()))
    },
    reader: |s| s.mo_fence(),
};

#[allow(const_err)]
const _PROP_SEPARATOR: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().separator()))
    },
    reader: |s| s.mo_separator(),
};

#[allow(const_err)]
const PROP_LSPACE: Property<Length, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || Length::SpaceLevel(SpaceLevel::ThickMathSpace),
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(Length::SpaceLevel(v.lspace().clone())))
    },
    reader: |s| s.mo_lspace(),
};

#[allow(const_err)]
const PROP_RSPACE: Property<Length, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || Length::SpaceLevel(SpaceLevel::ThickMathSpace),
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(Length::SpaceLevel(v.rspace().clone())))
    },
    reader: |s| s.mo_rspace(),
};

#[allow(const_err)]
const PROP_STRETCHY: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().stretchy()))
    },
    reader: |s| s.mo_stretchy(),
};

#[allow(const_err)]
const PROP_SYMMETRIC: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().symmetric()))
    },
    reader: |s| s.mo_symmetric(),
};

#[allow(const_err)]
const PROP_MAX_SIZE: Property<Length, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || Length::Infinity,
    computer: |_, _, _, _| {
        None
    },
    reader: |s| s.mo_max_size(),
};

#[allow(const_err)]
const PROP_MIN_SIZE: Property<Length, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || Length::EM(1.),
    computer: |_, _, _, _| {
        None
    },
    reader: |s| s.mo_min_size(),
};

#[allow(const_err)]
const PROP_LARGE_OP: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().largeop()))
    },
    reader: |s| s.mo_large_op(),
};

#[allow(const_err)]
const PROP_MOVABLE_LIMITS: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().movable_limits()))
    },
    reader: |s| s.mo_movable_limits(),
};

#[allow(const_err)]
const PROP_ACCENT: Property<bool, Mo, FormRequiredComputationContext> = Property::Computed {
    default: || false,
    computer: |ctx, element, _, computation_ctx| {
        let operator = element.get_text();
        let op_dict = ctx.operator_dictionary();

        op_dict.operator_attrs_approx(operator, computation_ctx.get_form())
            .and_then(|v| Some(v.properties().accent()))
    },
    reader: |s| s.mo_accent(),
};

pub struct Mo {
    instance_id: InstanceId,

    form: Option<OperatorForm>,
    fence: Option<bool>,
    separator: Option<bool>,
    lspace: Option<Length>,
    rspace: Option<Length>,
    stretchy: Option<bool>,
    symmetric: Option<bool>,
    max_size: Option<Length>,
    min_size: Option<Length>,
    large_op: Option<bool>,
    movable_limits: Option<bool>,
    accent: Option<bool>,

    token_props: SpecifiedTokenProps,
    presentation_props: SpecifiedPresentationProps,
}

impl Mo {
    pub fn new(text: String) -> Mo {
        Mo {
            instance_id: InstanceId::new(),

            form: None,
            fence: None,
            separator: None,
            lspace: None,
            rspace: None,
            stretchy: None,
            symmetric: None,
            max_size: None,
            min_size: None,
            large_op: None,
            movable_limits: None,
            accent: None,

            token_props: SpecifiedTokenProps {
                text: Rc::new(text),
                math_variant: None,
                math_size: None,
                dir: None,
            },

            presentation_props: SpecifiedPresentationProps {
                math_color: None,
                math_background: None,
            },
        }
    }

    pub fn with_form<'a>(&'a mut self, form: Option<OperatorForm>) -> &'a mut Self {
        self.form = form;
        self
    }

    pub fn get_form(&self) -> Option<&OperatorForm> {
        self.form.as_ref()
    }

    pub fn with_fence<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.fence = form;
        self
    }

    pub fn get_fence(&self) -> Option<&bool> {
        self.fence.as_ref()
    }

    pub fn with_separator<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.separator = form;
        self
    }

    pub fn get_separator(&self) -> Option<&bool> {
        self.separator.as_ref()
    }

    pub fn with_lspace<'a>(&'a mut self, form: Option<Length>) -> &'a mut Self {
        self.lspace = form;
        self
    }

    pub fn get_lspace(&self) -> Option<&Length> {
        self.lspace.as_ref()
    }

    pub fn with_rspace<'a>(&'a mut self, form: Option<Length>) -> &'a mut Self {
        self.rspace = form;
        self
    }

    pub fn get_rspace(&self) -> Option<&Length> {
        self.rspace.as_ref()
    }


    pub fn with_stretchy<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.stretchy = form;
        self
    }

    pub fn get_stretchy(&self) -> Option<&bool> {
        self.stretchy.as_ref()
    }

    pub fn with_symmetric<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.symmetric = form;
        self
    }

    pub fn get_symmetric(&self) -> Option<&bool> {
        self.symmetric.as_ref()
    }

    pub fn with_max_size<'a>(&'a mut self, form: Option<Length>) -> &'a mut Self {
        self.max_size = form;
        self
    }

    pub fn get_max_size(&self) -> Option<&Length> {
        self.max_size.as_ref()
    }
    
    pub fn with_min_size<'a>(&'a mut self, form: Option<Length>) -> &'a mut Self {
        self.min_size = form;
        self
    }

    pub fn get_min_size(&self) -> Option<&Length> {
        self.min_size.as_ref()
    }
    
    pub fn with_large_op<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.large_op = form;
        self
    }

    pub fn get_large_op(&self) -> Option<&bool> {
        self.large_op.as_ref()
    }

    pub fn with_movable_limits<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.movable_limits = form;
        self
    }

    pub fn get_movable_limits(&self) -> Option<&bool> {
        self.movable_limits.as_ref()
    }

    pub fn with_accent<'a>(&'a mut self, form: Option<bool>) -> &'a mut Self {
        self.accent = form;
        self
    }

    pub fn get_accent(&self) -> Option<&bool> {
        self.accent.as_ref()
    }

}

impl Element for Mo {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
              style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let token_layout = self.layout_token_element(context, &mut calculator);

        let form = calculator.calculate(&PROP_FORM, self.form.as_ref());
        let compute_ctx = FormRequiredComputationContext::new(form.clone());

        let font_size = token_layout.math_size;

        Box::new(MoLayout {
            lspace: calculator.calculate_contextual(
                &PROP_LSPACE, self.lspace.as_ref(), &compute_ctx
            ).get_length_du(context, font_size),
            rspace: calculator.calculate_contextual(
                &PROP_RSPACE, self.rspace.as_ref(), &compute_ctx
            ).get_length_du(context, font_size),
            stretchy: calculator.calculate_contextual(
                &PROP_STRETCHY, self.stretchy.as_ref(), &compute_ctx),
            symmetric: calculator.calculate_contextual(
                &PROP_SYMMETRIC, self.symmetric.as_ref(), &compute_ctx),
            max_size: calculator.calculate_contextual(
                &PROP_MAX_SIZE, self.max_size.as_ref(), &compute_ctx
            ).get_length_du(context, font_size),
            min_size: calculator.calculate_contextual(
                &PROP_MIN_SIZE, self.min_size.as_ref(), &compute_ctx
            ).get_length_du(context, font_size),
            large_op: calculator.calculate_contextual(
                &PROP_LARGE_OP, self.large_op.as_ref(), &compute_ctx),
            movable_limits: calculator.calculate_contextual(
                &PROP_MOVABLE_LIMITS, self.movable_limits.as_ref(), &compute_ctx),
            accent: calculator.calculate_contextual(
                &PROP_ACCENT, self.accent.as_ref(), &compute_ctx),

            token_element: token_layout,
        })
    }

    fn type_info(&self) -> ElementType {
        ElementType::TokenElement(TokenElement::Mo)
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

impl PresentationPrivate<Mo> for Mo {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl TokenPrivate<Mo> for Mo {
    fn get_specified_token_props(&self) -> &SpecifiedTokenProps {
        &self.token_props
    }

    fn get_specified_token_props_mut(&mut self) -> &mut SpecifiedTokenProps {
        &mut self.token_props
    }
}

impl Token<Mo> for Mo {}

impl Presentation<Mo> for Mo {}


#[cfg(test)]
mod test {
    use super::*;
    use ::elements::*;
    use ::test::skia::Snapshot;

    #[test]
    fn it_works() {
        let snap = Snapshot::default();
        snap.snap_element(&Mo::new(String::from("+")), "mo_default");

        let mut row = Mrow::new();

        row.with_child(Box::new(Mn::new(String::from("2"))));
        row.with_child(Box::new(Mi::new(String::from("x"))));
        row.with_child(Box::new(Mo::new(String::from("+"))));
        row.with_child(Box::new(Mn::new(String::from("3"))));

        snap.snap_element(&row, "mo_simple_row");
    }
}