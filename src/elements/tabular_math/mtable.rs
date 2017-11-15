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

use super::super::{Element, Property, EmptyComputeCtx, Family, InheritedProps, StyleProps,
                   InstanceId, ElementType, TablularMath, SpecifiedPresentationProps,
                   PresentationPrivate, Presentation, PropertyCalculator};
use ::props::{TableVAlign, VAlign, HAlign, Length, LineType, FrameSpacing, DisplayStyle, TableSide,
              GroupAlign, ColumnWidth, MathSize};
use super::{Mlabeledtr};
use ::platform::{Context};
use ::layout::{Layout, MtableLayout};
use ::utils::get_variable_length_prop;

#[allow(const_err)]
const PROP_ALIGN: Property<TableVAlign, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || TableVAlign::new(VAlign::Axis, None),
    reader: |i| i.mtable_align(),
};

#[allow(const_err)]
const PROP_ROW_ALIGN: Property<Vec<VAlign>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![VAlign::Baseline],
    reader: |i| i.mtable_row_align(),
};

#[allow(const_err)]
const PROP_COLUMN_ALIGN: Property<Vec<HAlign>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![HAlign::Center],
    reader: |i| i.mtable_column_align(),
};

#[allow(const_err)]
const PROP_GROUP_ALIGN: Property<Vec<Vec<GroupAlign>>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![vec![GroupAlign::Left]],
    reader: |i| i.mtable_group_align(),
};

#[allow(const_err)]
const PROP_ALIGNMENT_SCOPE: Property<Vec<bool>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![true],
    reader: |i| i.mtable_alignment_scope(),
};

#[allow(const_err)]
const PROP_COLUMN_WIDTH: Property<Vec<ColumnWidth>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![ColumnWidth::Auto],
    reader: |i| i.mtable_column_width(),
};

#[allow(const_err)]
const PROP_WIDTH: Property<Length, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || Length::Auto,
    reader: |i| i.mtable_width(),
};

#[allow(const_err)]
const PROP_ROW_SPACING: Property<Vec<Length>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![Length::EM(0.5)],
    reader: |i| i.mtable_row_spacing(),
};

#[allow(const_err)]
const PROP_COLUMN_SPACING: Property<Vec<Length>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![Length::EM(0.8)],
    reader: |i| i.mtable_column_spacing(),
};

#[allow(const_err)]
const PROP_ROW_LINES: Property<Vec<LineType>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![LineType::None],
    reader: |i| i.mtable_row_lines(),
};

#[allow(const_err)]
const PROP_COLUMN_LINES: Property<Vec<LineType>, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || vec![LineType::None],
    reader: |i| i.mtable_column_lines(),
};

#[allow(const_err)]
const PROP_FRAME: Property<LineType, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || LineType::None,
    reader: |i| i.mtable_frame(),
};

#[allow(const_err)]
const PROP_FRAME_SPACING: Property<FrameSpacing, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || FrameSpacing::new(Length::EM(0.4), Length::EX(0.5)),
    reader: |i| i.mtable_frame_spacing(),
};

#[allow(const_err)]
const PROP_EQUAL_ROWS: Property<bool, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || false,
    reader: |i| i.mtable_equal_rows(),
};

#[allow(const_err)]
const PROP_EQUAL_COLUMNS: Property<bool, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || false,
    reader: |i| i.mtable_equal_columns(),
};

#[allow(const_err)]
const PROP_DISPLAY_STYLE: Property<DisplayStyle, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || false,
    reader: |i| i.mtable_display_style(),
};

#[allow(const_err)]
const PROP_SIDE: Property<TableSide, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || TableSide::Right,
    reader: |i| i.mtable_side(),
};

#[allow(const_err)]
const PROP_MIN_LABEL_SPACING: Property<Length, Mtable, EmptyComputeCtx> = Property::Specified {
    default: || Length::EM(0.8),
    reader: |i| i.mtable_min_label_spacing(),
};


pub struct Mtable {
    align: Option<TableVAlign>,
    row_align: Option<Vec<VAlign>>,
    column_align: Option<Vec<HAlign>>,
    group_align: Option<Vec<Vec<GroupAlign>>>,
    alignment_scope: Option<Vec<bool>>,
    column_width: Option<Vec<ColumnWidth>>,
    width: Option<Length>,
    row_spacing: Option<Vec<Length>>,
    column_spacing: Option<Vec<Length>>,
    row_lines: Option<Vec<LineType>>,
    column_lines: Option<Vec<LineType>>,
    frame: Option<LineType>,
    frame_spacing: Option<FrameSpacing>,
    equal_rows: Option<bool>,
    equal_columns: Option<bool>,
    display_style: Option<DisplayStyle>,
    side: Option<TableSide>,
    min_label_spacing: Option<Length>,

    rows: Vec<Mlabeledtr>,

    presentation_props: SpecifiedPresentationProps,
    instance_id: InstanceId,
}

impl Mtable {
    pub fn new() -> Mtable {
        Mtable {
            align: None,
            row_align: None,
            column_align: None,
            group_align: None,
            alignment_scope: None,
            column_width: None,
            width: None,
            row_spacing: None,
            column_spacing: None,
            row_lines: None,
            column_lines: None,
            frame: None,
            frame_spacing: None,
            equal_rows: None,
            equal_columns: None,
            display_style: None,
            side: None,
            min_label_spacing: None,
            rows: Vec::new(),
            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub fn with_row<'a>(&'a mut self, row: Mlabeledtr) -> &'a mut Mtable {
        self.rows.push(row);
        self
    }
    
    pub fn rows(&self) -> &Vec<Mlabeledtr> {
        &self.rows
    }
    
    pub fn with_align<'a>(&'a mut self, align: Option<TableVAlign>) -> &'a mut Mtable {
        self.align = align;
        self
    }
    
    pub fn align(&self) -> Option<&TableVAlign> {
        self.align.as_ref()
    }

    pub fn with_row_align<'a>(&'a mut self, row_align: Option<Vec<VAlign>>) -> &'a mut Mtable {
        self.row_align = row_align;
        self
    }

    pub fn row_align(&self) -> Option<&Vec<VAlign>> {
        self.row_align.as_ref()
    }

    pub fn with_column_align<'a>(&'a mut self, column_align: Option<Vec<HAlign>>) -> &'a mut Mtable {
        self.column_align = column_align;
        self
    }

    pub fn column_align(&self) -> Option<&Vec<HAlign>> {
        self.column_align.as_ref()
    }

    pub fn with_group_align<'a>(&'a mut self, group_align: Option<Vec<Vec<GroupAlign>>>) -> &'a mut Mtable {
        self.group_align = group_align;
        self
    }

    pub fn group_align(&self) -> Option<&Vec<Vec<GroupAlign>>> {
        self.group_align.as_ref()
    }

    pub fn with_alignment_scope<'a>(&'a mut self, alignment_scope: Option<Vec<bool>>) -> &'a mut Mtable {
        self.alignment_scope = alignment_scope;
        self
    }

    pub fn alignment_scope(&self) -> Option<&Vec<bool>> {
        self.alignment_scope.as_ref()
    }

    pub fn with_column_width<'a>(&'a mut self, column_width: Option<Vec<ColumnWidth>>) -> &'a mut Mtable {
        self.column_width = column_width;
        self
    }

    pub fn column_width(&self) -> Option<&Vec<ColumnWidth>> {
        self.column_width.as_ref()
    }

    pub fn with_width<'a>(&'a mut self, width: Option<Length>) -> &'a mut Mtable {
        self.width = width;
        self
    }

    pub fn width(&self) -> Option<&Length> {
        self.width.as_ref()
    }

    pub fn with_row_spacing<'a>(&'a mut self, row_spacing: Option<Vec<Length>>) -> &'a mut Mtable {
        self.row_spacing = row_spacing;
        self
    }

    pub fn row_spacing(&self) -> Option<&Vec<Length>> {
        self.row_spacing.as_ref()
    }

    pub fn with_column_spacing<'a>(&'a mut self, column_spacing: Option<Vec<Length>>) -> &'a mut Mtable {
        self.column_spacing = column_spacing;
        self
    }

    pub fn column_spacing(&self) -> Option<&Vec<Length>> {
        self.column_spacing.as_ref()
    }

    pub fn with_row_lines<'a>(&'a mut self, row_lines: Option<Vec<LineType>>) -> &'a mut Mtable {
        self.row_lines = row_lines;
        self
    }

    pub fn row_lines(&self) -> Option<&Vec<LineType>> {
        self.row_lines.as_ref()
    }

    pub fn with_column_lines<'a>(&'a mut self, column_lines: Option<Vec<LineType>>) -> &'a mut Mtable {
        self.column_lines = column_lines;
        self
    }

    pub fn column_lines(&self) -> Option<&Vec<LineType>> {
        self.column_lines.as_ref()
    }

    pub fn with_frame<'a>(&'a mut self, frame: Option<LineType>) -> &'a mut Mtable {
        self.frame = frame;
        self
    }

    pub fn frame(&self) -> Option<&LineType> {
        self.frame.as_ref()
    }

    pub fn with_frame_spacing<'a>(&'a mut self, frame_spacing: Option<FrameSpacing>) -> &'a mut Mtable {
        self.frame_spacing = frame_spacing;
        self
    }

    pub fn frame_spacing(&self) -> Option<&FrameSpacing> {
        self.frame_spacing.as_ref()
    }

    pub fn with_equal_rows<'a>(&'a mut self, equal_rows: Option<bool>) -> &'a mut Mtable {
        self.equal_rows = equal_rows;
        self
    }

    pub fn equal_rows(&self) -> Option<&bool> {
        self.equal_rows.as_ref()
    }

    pub fn with_equal_columns<'a>(&'a mut self, equal_columns: Option<bool>) -> &'a mut Mtable {
        self.equal_columns = equal_columns;
        self
    }

    pub fn equal_columns(&self) -> Option<&bool> {
        self.equal_columns.as_ref()
    }

    pub fn with_display_style<'a>(&'a mut self, display_style: Option<DisplayStyle>) -> &'a mut Mtable {
        self.display_style = display_style;
        self
    }

    pub fn display_style(&self) -> Option<&DisplayStyle> {
        self.display_style.as_ref()
    }

    pub fn with_side<'a>(&'a mut self, side: Option<TableSide>) -> &'a mut Mtable {
        self.side = side;
        self
    }

    pub fn side(&self) -> Option<&TableSide> {
        self.side.as_ref()
    }

    pub fn with_min_label_spacing<'a>(&'a mut self, min_label_spacing: Option<Length>) -> &'a mut Mtable {
        self.min_label_spacing = min_label_spacing;
        self
    }

    pub fn min_label_spacing(&self) -> Option<&Length> {
        self.min_label_spacing.as_ref()
    }
}

impl Element for Mtable {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let align = calculator.calculate(
            &PROP_ALIGN, self.align.as_ref());
        let row_align = calculator.calculate(
            &PROP_ROW_ALIGN, self.row_align.as_ref());
        let column_align = calculator.calculate(
            &PROP_COLUMN_ALIGN, self.column_align.as_ref());
        let group_align = calculator.calculate(
            &PROP_GROUP_ALIGN, self.group_align.as_ref());
        let alignment_scope = calculator.calculate(
            &PROP_ALIGNMENT_SCOPE, self.alignment_scope.as_ref());
        let column_width = calculator.calculate(
            &PROP_COLUMN_WIDTH, self.column_width.as_ref());
        let width = calculator.calculate(
            &PROP_WIDTH, self.width.as_ref());
        let row_spacing = calculator.calculate(
            &PROP_ROW_SPACING, self.row_spacing.as_ref());
        let column_spacing = calculator.calculate(
            &PROP_COLUMN_SPACING, self.column_spacing.as_ref());
        let row_lines = calculator.calculate(
            &PROP_ROW_LINES, self.row_lines.as_ref());
        let column_lines = calculator.calculate(
            &PROP_COLUMN_LINES, self.column_lines.as_ref());
        let frame = calculator.calculate(&PROP_FRAME, self.frame.as_ref());
        let frame_spacing = calculator.calculate(
            &PROP_FRAME_SPACING, self.frame_spacing.as_ref());
        let equal_rows = calculator.calculate(
            &PROP_EQUAL_COLUMNS, self.equal_rows.as_ref());
        let equal_columns = calculator.calculate(
            &PROP_EQUAL_COLUMNS, self.equal_columns.as_ref());
        let display_style = calculator.calculate(
            &PROP_DISPLAY_STYLE, self.display_style.as_ref());
        let side = calculator.calculate(
            &PROP_SIDE, self.side.as_ref());
        let min_label_spacing = calculator.calculate(
            &PROP_MIN_LABEL_SPACING, self.min_label_spacing.as_ref());

        let new_family = family.add(self);
        let base_size = presentation_layout.script_level.get_font_size(context, &MathSize::NORMAL);

        let mut inherited_fork = calculator.make_fork();
        inherited_fork
            .table_column_align(column_align)
            .table_group_align(group_align);

        let inherited_fork = inherited_fork.copy();

        Box::new(MtableLayout {
            base_size,
            align,
            column_width,
            width,
            row_spacing,
            column_spacing,
            row_lines,
            column_lines,
            frame,
            frame_spacing,
            equal_rows,
            equal_columns,
            side,
            min_label_spacing,
            rows: self.rows.iter().enumerate()
                .map(|(index, row)| {
                    let row_align = get_variable_length_prop(&row_align, index);
                    let mut inherited = inherited_fork.copier();
                    inherited.table_row_align(row_align.clone());
                    row.layout_concrete(context, &new_family, &inherited.copy(), style)
                })
                .collect(),
            presentation_layout,
        })

    }

    fn type_info(&self) -> ElementType {
        ElementType::TabularMath(TablularMath::Mtable)
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

impl PresentationPrivate<Mtable> for Mtable {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mtable> for Mtable {}