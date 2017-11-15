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

use ::props::{HAlign, VAlign, GroupAlign};
use super::super::{Element, SpecifiedPresentationProps, Presentation, PresentationPrivate,
                   InheritedProps, StyleProps, Family, ElementType, InstanceId, TablularMath, EmptyComputeCtx, Property};
use ::platform::{Context};
use ::layout::{Layout};

#[allow(const_err)]
const PROP_ROW_ALIGN: Property<VAlign, Mtd, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.table_row_align(),
    writer: |val, i| i.table_row_align(val),
};

#[allow(const_err)]
const PROP_COLUMN_ALIGN: Property<HAlign, Mtd, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.table_mtd_column_align(),
    writer: |val, i| i.table_mtd_column_align(val),
};

#[allow(const_err)]
const PROP_GROUP_ALIGN: Property<Vec<GroupAlign>, Mtd, EmptyComputeCtx> = Property::Inherited {
    reader: |i| i.table_mtd_group_align(),
    writer: |val, i| i.table_mtd_group_align(val),
};

pub struct Mtd {
    row_span: Option<u32>,
    column_span: Option<u32>,
    row_align: Option<VAlign>,
    column_align: Option<HAlign>,
    group_align: Option<Vec<GroupAlign>>,

    child: Option<Box<Element>>,

    presentation_props: SpecifiedPresentationProps,

    instance_id: InstanceId,
}

impl Mtd {
    pub fn new(child: Option<Box<Element>>) -> Mtd {
        Mtd {
            row_span: None,
            column_span: None,
            row_align: None,
            column_align: None,
            group_align: None,
            child,
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub fn with_child<'a>(&'a mut self, child: Option<Box<Element>>) -> &'a mut Mtd {
        self.child = child;
        self
    }

    pub fn child(&self) -> Option<&Box<Element>> {
        self.child.as_ref()
    }

    pub fn with_row_span<'a>(&'a mut self, row_span: Option<u32>) -> &'a mut Mtd {
        self.row_span = row_span;
        self
    }

    pub fn row_span(&self) -> Option<&u32> {
        self.row_span.as_ref()
    }

    pub fn with_column_span<'a>(&'a mut self, column_span: Option<u32>) -> &'a mut Mtd {
        self.column_span = column_span;
        self
    }

    pub fn column_span(&self) -> Option<&u32> {
        self.column_span.as_ref()
    }

    pub fn with_row_align<'a>(&'a mut self, row_align: Option<VAlign>) -> &'a mut Mtd {
        self.row_align = row_align;
        self
    }

    pub fn row_align(&self) -> Option<&VAlign> {
        self.row_align.as_ref()
    }

    pub fn with_column_align<'a>(&'a mut self, column_align: Option<HAlign>) -> &'a mut Mtd {
        self.column_align = column_align;
        self
    }

    pub fn column_align(&self) -> Option<&HAlign> {
        self.column_align.as_ref()
    }

    pub fn with_group_align<'a>(&'a mut self, group_align: Option<Vec<GroupAlign>>) -> &'a mut Mtd {
        self.group_align = group_align;
        self
    }

    pub fn group_align(&self) -> Option<&Vec<GroupAlign >> {
        self.group_align.as_ref()
    }
}

impl Element for Mtd {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        unimplemented!()
    }

    fn type_info(&self) -> ElementType {
        ElementType::TabularMath(TablularMath::Mtd)
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

impl PresentationPrivate<Mtd> for Mtd {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mtd> for Mtd {}