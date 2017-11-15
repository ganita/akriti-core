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
use std::collections::HashSet;

use ::cassowary::{self, Solver, Variable, Expression};
use cassowary::WeightedRelation::*;
use cassowary::strength::*;

use super::super::{Layout, ConcreteLayout, PresentationLayout};
use super::{MlabeledtrLayout, MlabeledtrDrawable, MtdDrawable};
use ::draw::{Drawable, MeasureMode, BoundingBox};
use ::platform::{Context};
use ::paint::{Point, Canvas};
use ::utils::get_variable_length_prop;

use ::props::{TableVAlign, ColumnWidth, Length, LineType, FrameSpacing, TableSide};


pub struct MtableLayout {
    pub(crate) base_size: f32,
    pub(crate) align: TableVAlign,
    pub(crate) column_width: Vec<ColumnWidth>,
    pub(crate) width: Length,
    pub(crate) row_spacing: Vec<Length>,
    pub(crate) column_spacing: Vec<Length>,
    pub(crate) row_lines: Vec<LineType>,
    pub(crate) column_lines: Vec<LineType>,
    pub(crate) frame: LineType,
    pub(crate) frame_spacing: FrameSpacing,
    pub(crate) equal_rows: bool,
    pub(crate) equal_columns: bool,
    pub(crate) side: TableSide,
    pub(crate) min_label_spacing: Length,

    pub(crate) rows: Vec<MlabeledtrLayout>,
    pub(crate) presentation_layout: PresentationLayout,
}

impl Layout for MtableLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        let mut wrapper = self.presentation_layout.layout(context);

        wrapper.wrap(MtableDrawable {
            layout: self,
            rows: self.rows.iter()
                .map(|row| ConcreteLayout::layout(row, context))
                .collect(),
            base_size: self.base_size,
            align: self.align.clone(),
            column_width_reader: |s| &s.column_width,
            width: self.width.clone(),
            row_spacing_reader: |s| &s.row_spacing,
            col_spacing_reader: |s| &s.column_spacing,
            row_lines_reader: |s| &s.row_lines,
            col_lines_reader: |s| &s.column_lines,
            frame: self.frame.clone(),
            frame_spacing: self.frame_spacing.clone(),
            equal_rows: self.equal_rows,
            equal_cols: self.equal_columns,
            label_side: self.side.clone(),
            min_label_spacing: self.min_label_spacing.clone(),
        });

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        Box::new(wrapper)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}


type ColumnWidthReader = fn(&MtableLayout) -> &Vec<ColumnWidth>;
type SpacingReader = fn(&MtableLayout) -> &Vec<Length>;
type LineTypeReader = fn(&MtableLayout) -> &Vec<LineType>;

pub(in super::super::tabular_math) struct MtableDrawable<'a> {
    pub(in super::super::tabular_math) layout: &'a MtableLayout,
    pub(in super::super::tabular_math) rows: Vec<MlabeledtrDrawable<'a>>,

    pub(in super::super::tabular_math) base_size: f32,
    pub(in super::super::tabular_math) align: TableVAlign,
    pub(in super::super::tabular_math) column_width_reader: ColumnWidthReader,
    pub(in super::super::tabular_math) width: Length,
    pub(in super::super::tabular_math) row_spacing_reader: SpacingReader,
    pub(in super::super::tabular_math) col_spacing_reader: SpacingReader,
    pub(in super::super::tabular_math) row_lines_reader: LineTypeReader,
    pub(in super::super::tabular_math) col_lines_reader: LineTypeReader,
    pub(in super::super::tabular_math) frame: LineType,
    pub(in super::super::tabular_math) frame_spacing: FrameSpacing,
    pub(in super::super::tabular_math) equal_rows: bool,
    pub(in super::super::tabular_math) equal_cols: bool,
    pub(in super::super::tabular_math) label_side: TableSide,
    pub(in super::super::tabular_math) min_label_spacing: Length
}

impl<'a> Drawable for MtableDrawable<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        unimplemented!()
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        let mut constraints = Vec::new();
        let table_width = Variable::new();

        let specified_table_width = self.width.get_length_du(context, self.base_size);
        if !specified_table_width.is_nan() {
            constraints.push(table_width | EQ(REQUIRED) | specified_table_width);
        }

        let mut grid = Grid::new();

        for (row_no, row) in self.rows.iter().enumerate() {
            let mut cell = Cell::new(row_no, 0);
            for column in row.column.iter() {
                let column: &MtdDrawable = column;
                cell = grid.insert(column, cell);
            }
        }

        for (col_no, col) in grid.columns().iter().enumerate() {
            for child in col.children.iter() {
                let child: &MtdDrawable = child;
                let required_width = child.bounding_box().width();
                let mut col_constraint = col.span + 0f32;
                for i in 1..child.row_span {
                    col_constraint = col_constraint + grid.columns()[col_no + i as usize].span;
                }
                constraints.push(col_constraint | GE(STRONG) | required_width);
            }
        }

        let column_width = (self.column_width_reader)(self.layout);
        for i in 0..grid.col_count() {
            let col_width = get_variable_length_prop(column_width, i as usize);
            match *col_width {
                ColumnWidth::Length(ref val) => {
                    let len = val.get_length_du(context, self.base_size);
                    if !len.is_nan() {
                        constraints.push(grid.columns()[i].span | EQ(REQUIRED) | len);
                    }
                },
                ColumnWidth::Fit => {
                    // TODO fit
                },
                ColumnWidth::Auto => {
                    // TODO fit
                }
            }

            if self.equal_cols && i < grid.col_count()-1 {
                constraints.push(grid.columns()[i].span | EQ(REQUIRED) | grid.columns()[i+1].span);
            }
        }

        unimplemented!()
    }

    fn bounding_box(&self) -> &BoundingBox {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Cell {
    row_no: usize,
    col_no: usize,
}

impl Cell {
    pub fn new(row_no: usize, col_no: usize) -> Cell {
        Cell { row_no, col_no }
    }

    pub fn inc_col(&self, by: usize) -> Cell {
        Cell { row_no: self.row_no, col_no: self.col_no + by }
    }

    pub fn inc_row(&self, by: usize) -> Cell {
        Cell { row_no: self.row_no + by, col_no: self.col_no }
    }
}

struct Grid<'a> {
    occupancy_register: HashSet<Cell>,
    rows: Vec<GridBox<'a>>,
    columns: Vec<GridBox<'a>>,
    row_count: usize,
    col_count: usize,
}

impl<'a> Grid<'a> {
    pub fn new() -> Grid<'a> {
        Grid {
            occupancy_register: HashSet::new(),
            rows: Vec::new(),
            columns: Vec::new(),
            row_count: 0,
            col_count: 0,
        }
    }

    pub fn insert(&mut self, drawable: &'a MtdDrawable<'a>, cell: Cell) -> Cell {
        let mut cell = cell;

        while !self.occupancy_register.contains(&cell) {
            cell = cell.inc_col(1);
        }

        for i in 0..drawable.row_span {
            for j in 0..drawable.col_span {
                self.occupancy_register.insert(cell.inc_col(i as usize).inc_row(j as usize));
            }
        }

        if self.rows.get(cell.row_no).is_none() {
            self.rows.insert(cell.row_no, GridBox::new());
        }
        self.rows.get_mut(cell.row_no).unwrap().insert(cell.col_no, drawable);

        if self.columns.get(cell.col_no).is_none() {
            self.columns.insert(cell.col_no, GridBox::new());
        }
        self.columns.get_mut(cell.col_no).unwrap().insert(cell.row_no, drawable);

        self.row_count = self.row_count.max(cell.row_no + drawable.col_span as usize);
        self.col_count = self.col_count.max(cell.col_no + drawable.row_span as usize);

        cell
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn col_count(&self) -> usize {
        self.col_count
    }

    pub fn rows(&self) -> &Vec<GridBox<'a>> {
        &self.rows
    }

    pub fn columns(&self) -> &Vec<GridBox<'a>> {
        &self.columns
    }
}

struct GridBox<'a> {
    children: Vec<&'a MtdDrawable<'a>>,
    span: Variable,
}

impl<'a> GridBox<'a> {
    pub fn new() -> GridBox<'a> {
        GridBox { children: Vec::new(), span: Variable::new() }
    }

    pub fn insert(&mut self, index: usize, child: &'a MtdDrawable<'a>) {
        self.children.insert(index, child);
    }
}
