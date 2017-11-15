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

use super::super::{Element, ElementType, Presentation, PresentationPrivate, SpecifiedPresentationProps,
                   InstanceId, InheritedProps, StyleProps, Family, TablularMath, Property,
                   PropertyCalculator, EmptyComputeCtx};
use super::{Mtd};
use ::props::{VAlign, HAlign, GroupAlign};
use ::platform::Context;
use ::layout::{Layout, MlabeledtrLayout};
use ::utils::get_variable_length_prop;

#[allow(const_err)]
const PROP_ROW_ALIGN: Property<VAlign, Mlabeledtr, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.table_row_align(),
    writer: |val, i| i.table_row_align(val),
};

#[allow(const_err)]
const PROP_COLUMN_ALIGN: Property<Vec<HAlign>, Mlabeledtr, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.table_column_align(),
    writer: |val, i| i.table_column_align(val),
};

#[allow(const_err)]
const PROP_GROUP_ALIGN: Property<Vec<Vec<GroupAlign>>, Mlabeledtr, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.table_group_align(),
    writer: |val, i| i.table_group_align(val),
};

pub struct Mlabeledtr {
    row_align: Option<VAlign>,
    column_align: Option<Vec<HAlign>>,
    group_align: Option<Vec<Vec<GroupAlign>>>,

    label: Mtd,
    children: Vec<Mtd>,

    presentation_props: SpecifiedPresentationProps,
    instance_id: InstanceId,
}

impl Mlabeledtr {
    pub fn new(label: Mtd) -> Mlabeledtr {
        Mlabeledtr {
            row_align: None,
            column_align: None,
            group_align: None,
            label,
            children: Vec::new(),
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub(crate) fn layout_concrete<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                                  style: &Option<&StyleProps>) -> MlabeledtrLayout {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());
        let presentation_layout = self.layout_presentation(&mut calculator);

        let _row_align = calculator.calculate(
            &PROP_ROW_ALIGN, self.row_align.as_ref());
        let column_align = calculator.calculate(
            &PROP_COLUMN_ALIGN, self.column_align.as_ref());
        let group_align = calculator.calculate(
            &PROP_GROUP_ALIGN, self.group_align.as_ref());

        let new_family = family.add(self);
        let inherited_fork = calculator.make_fork().copy();

        MlabeledtrLayout {
            children: self.children.iter().enumerate().map(|(index, mtd)| {
                let mut fork = inherited_fork.copier();
                fork.table_mtd_column_align(get_variable_length_prop(&column_align, index).clone());
                fork.table_mtd_group_align(get_variable_length_prop(&group_align, index).clone());

                mtd.layout_concrete(context, &new_family, &fork.copy(), style)
            }).collect(),
            presentation_layout
        }
    }

    pub fn with_mtd<'a>(&'a mut self, mtd: Mtd) -> &'a mut Mlabeledtr {
        self.children.push(mtd);
        self
    }

    pub fn children(&self) -> &Vec<Mtd> {
        &self.children
    }

    pub fn with_row_align<'a>(&'a mut self, row_align: Option<VAlign>) -> &'a mut Mlabeledtr {
        self.row_align = row_align;
        self
    }

    pub fn row_align(&self) -> Option<&VAlign> {
        self.row_align.as_ref()
    }

    pub fn with_column_align<'a>(&'a mut self, column_align: Option<Vec<HAlign>>) -> &'a mut Mlabeledtr {
        self.column_align = column_align;
        self
    }

    pub fn column_align(&self) -> Option<&Vec<HAlign>> {
        self.column_align.as_ref()
    }

    pub fn with_group_align<'a>(&'a mut self, group_align: Option<Vec<Vec<GroupAlign>>>) -> &'a mut Mlabeledtr {
        self.group_align = group_align;
        self
    }

    pub fn group_align(&self) -> Option<&Vec<Vec<GroupAlign>>> {
        self.group_align.as_ref()
    }
}


impl Element for Mlabeledtr {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        Box::new(self.layout_concrete(context, family, inherited, style))
    }

    fn type_info(&self) -> ElementType {
        ElementType::TabularMath(TablularMath::Mlabeledtr)
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

impl PresentationPrivate<Mlabeledtr> for Mlabeledtr {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mlabeledtr> for Mlabeledtr {}