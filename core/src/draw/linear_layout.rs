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


use std::f32;

use super::{Drawable, BoundingBox, MeasureMode};
use ::platform::Context;
use ::paint::{Canvas, Point, Rect};

pub struct LinearLayout<'a> {
    children: Vec<Child<'a>>,
    pub gravity: Gravity,
    pub layout_align: Align,

    bounding_box: BoundingBox,
}

#[derive(Debug, PartialEq)]
pub enum Gravity {
    Vertical,
    Horizontal
}

#[derive(Debug, PartialEq, Clone)]
pub enum Align {
    Start,          // Top for horizontal and left for vertical layout
    End,            // Bottom for horizontal and right for vertical layout
    Center,
    Baseline,       // Affect only for horizontal layout. If set on vertical layout, center align will be used
    Axis            // Available only for horizontal layout. If set on vertical layout, center align will be used
}

#[derive(Debug, PartialEq)]
pub enum CrossAxisBoundMode {
    WrapContent,
    FillParent
}

pub struct LinearLayoutParams {
    align_self: Option<Align>,
    weight: f32,
    cross_axis_bound_mode: CrossAxisBoundMode,
}

impl LinearLayoutParams {
    pub fn new() -> LinearLayoutParams {
        LinearLayoutParams {
            align_self: None,
            weight: 0.0,
            cross_axis_bound_mode: CrossAxisBoundMode::WrapContent,
        }
    }

    pub fn with_align(mut self, align: Option<Align>) -> LinearLayoutParams {
        self.align_self = align;
        self
    }

    pub fn with_weight(mut self, weight: f32) -> LinearLayoutParams {
        self.weight = weight;
        self
    }

    pub fn with_cross_axis_bound_mode(mut self, mode: CrossAxisBoundMode) -> LinearLayoutParams {
        self.cross_axis_bound_mode = mode;
        self
    }
}

pub struct Child<'a> {
    drawable: Box<Drawable + 'a>,
    params: LinearLayoutParams,
    point: Point
}

struct AxisParams {
    align: Align,
    baseline_pos: f32,
    axis_baseline_shift: f32,
}

impl Default for AxisParams {
    fn default() -> Self {
        AxisParams {
            align: Align::Start,
            baseline_pos: 0.0,
            axis_baseline_shift: 0.0,
        }
    }
}

impl AxisParams {
    fn set_baseline_pos(&mut self, pos: f32) {
        self.baseline_pos = pos;
    }

    fn set_axis_pos(&mut self, pos: f32) {
        self.baseline_pos = pos - self.axis_baseline_shift;
    }

    pub fn baseline_pos(&self) -> f32 {
        self.baseline_pos
    }

    pub fn axis_pos(&self) -> f32 {
        self.baseline_pos + self.axis_baseline_shift
    }
}

impl<'a> Drawable for LinearLayout<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        for child in self.children.iter() {
            child.drawable.draw(canvas, &(pen_pos + &child.point));
        }
    }

    // TODO optimize time complexity to at least O(n^2)
    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        if self.children.len() == 0 {
            self.bounding_box = BoundingBox::default();
            return ();
        }

        if self.children.len() == 1 {
            self.children[0].drawable.calculate(context, &MeasureMode::Wrap,
                                                &MeasureMode::Wrap);
            self.bounding_box = self.children[0].drawable.bounding_box().clone();
            return ();
        }

        let layout_align = &self.layout_align;
        let layout_gravity = &self.gravity;

        let mut axis_params: Option<AxisParams> = None;
        let mut cross_axis_length = 0f32;

        // In first iteration we will compute the dimension of children to just wrap the content
        // and compute the total size of container
        for child in self.children.iter_mut() {
            // Compute minimum dimension required by child to wrap its contents
            child.drawable.calculate(context,&MeasureMode::Wrap, &MeasureMode::Wrap);

            let align = LinearLayout::get_align_self(layout_gravity,
                                                     layout_align,
                                                     child.params.align_self.as_ref());

            cross_axis_length = match *align {
                Align::Start | Align::End | Align::Center => {
                    let n_cross_axis_length = match self.gravity {
                        Gravity::Vertical => cross_axis_length.max(child.drawable.bounding_box().width()),
                        Gravity::Horizontal => cross_axis_length.max(child.drawable.bounding_box().height()),
                    };

                    if let Some(ref mut params) = axis_params {
                        params.baseline_pos = match params.align {
                            Align::Start => params.baseline_pos,

                            Align::Center | Align::Baseline | Align::Axis =>
                                ((n_cross_axis_length - cross_axis_length) / 2.) + params.baseline_pos,

                            Align::End => n_cross_axis_length - (cross_axis_length - params.baseline_pos),
                        };
                    }

                    n_cross_axis_length
                }
                Align::Baseline | Align::Axis => {
                    if self.gravity == Gravity::Vertical {
                        panic!("Cannot align to baseline or axis when using vertical gravity");
                    }

                    let (n_cross_axis_length, n_axis_pos) = if let Some(ref mut params) = axis_params {
                        let new_ascent = if *align == Align::Baseline {
                            params.baseline_pos().max(child.drawable.bounding_box().baseline_pos())
                        } else {
                            params.axis_pos().max(child.drawable.bounding_box().axis_pos())
                        };

                        let new_descent = if *align == Align::Baseline {
                            child.drawable.bounding_box().baseline().max(cross_axis_length - params.baseline_pos())
                        } else {
                            child.drawable.bounding_box().axis().max(cross_axis_length - params.axis_pos())
                        };

                        let basis_axis_pos = if *align == Align::Baseline {
                            params.baseline_pos()
                        } else {
                            params.axis_pos()
                        };

                        match params.align {
                            Align::Start => (cross_axis_length.max(basis_axis_pos + new_descent), basis_axis_pos),
                            Align::Center => (cross_axis_length, basis_axis_pos),
                            Align::End => (new_ascent + cross_axis_length - basis_axis_pos, new_ascent),
                            Align::Baseline => (new_ascent + new_descent, new_ascent),
                            Align::Axis => (cross_axis_length.max(new_ascent + new_descent), new_ascent)
                        }
                    } else {
                        (
                            cross_axis_length.max(child.drawable.bounding_box().height()),
                            if *align == Align::Baseline {
                                child.drawable.bounding_box().baseline_pos()
                            } else {
                                child.drawable.bounding_box().axis_pos()
                            }
                        )
                    };

                    if let Some(ref mut params) = axis_params {
                        if *align == Align::Baseline {
                            params.set_baseline_pos(n_axis_pos);
                        } else {
                            params.set_axis_pos(n_axis_pos);
                        }
                    }

                    n_cross_axis_length
                }
            };

            // If baseline and axis of layout is not set and if child is not having flex bound,
            // use the baseline and axis of that child as baseline and axis of layout
            if axis_params.is_none() && child.params.cross_axis_bound_mode != CrossAxisBoundMode::FillParent {
                axis_params = Some(AxisParams {
                    align: align.clone(),
                    baseline_pos: child.drawable.bounding_box().baseline_pos(),
                    axis_baseline_shift: child.drawable.bounding_box().axis_pos() -
                        child.drawable.bounding_box().baseline_pos(),
                })
            }
        }

        let mut axis_params = axis_params.unwrap_or(AxisParams {
            align: Align::Start,
            baseline_pos: cross_axis_length,
            axis_baseline_shift: 0.,
        });

        let mut main_axis_wrap_length = 0f32;
        let mut weight_sum = 0f32;
        let mut cross_axis_length_adjusted = cross_axis_length;
        // Calculate width and height of cross axis flexible items.
        for child in self.children.iter_mut() {
            if child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                let (width_mode, height_mode) = match self.gravity {
                    Gravity::Vertical => (MeasureMode::UpTo(cross_axis_length), MeasureMode::Wrap),
                    Gravity::Horizontal => (MeasureMode::Wrap, MeasureMode::UpTo(cross_axis_length)),
                };

                child.drawable.calculate(context, &width_mode, &height_mode);

                cross_axis_length_adjusted = cross_axis_length_adjusted
                    .max(child.drawable.bounding_box().height());
            }

            if child.params.weight <= 0. {
                main_axis_wrap_length += match self.gravity {
                    Gravity::Horizontal => child.drawable.bounding_box().width(),
                    Gravity::Vertical => child.drawable.bounding_box().height(),
                };
            }

            weight_sum += child.params.weight;
        }

        let cross_axis_diff = cross_axis_length_adjusted-cross_axis_length;
        let baseline_new = axis_params.baseline_pos() + cross_axis_diff;
        axis_params.set_baseline_pos(baseline_new);
        cross_axis_length = cross_axis_length_adjusted;

        let main_axis_available_length = match self.gravity {
            Gravity::Horizontal => if let MeasureMode::UpTo(width) = *width_mode { width } else { -1. },
            Gravity::Vertical => if let MeasureMode::UpTo(height) = *height_mode { height } else { -1. },
        } - main_axis_wrap_length ;

        let weight_factor = main_axis_available_length/weight_sum;

        let mut main_axis_pen = 0f32;
        for child in self.children.iter_mut() {

            // Stretch main axis flexible items
            if child.params.weight > 0. && weight_factor > 0. {
                let (width_mode, height_mode) = match self.gravity {
                    Gravity::Vertical => (
                        MeasureMode::UpTo(child.drawable.bounding_box().width()),
                        MeasureMode::UpTo(child.drawable.bounding_box().height()
                            .max(weight_factor*child.params.weight))
                    ),
                    Gravity::Horizontal => (
                        MeasureMode::UpTo(child.drawable.bounding_box().width()
                            .max(weight_factor*child.params.weight)),
                        MeasureMode::UpTo(child.drawable.bounding_box().height())
                    ),
                };

                child.drawable.calculate(context, &width_mode, &height_mode);
            }

            let align = if child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                Align::Start
            } else {
                LinearLayout::get_align_self(
                    &self.gravity,
                    &self.layout_align,
                    child.params.align_self.as_ref()
                ).clone()
            };

            let child_cross_axis_length = match self.gravity {
                Gravity::Vertical => child.drawable.bounding_box().width(),
                Gravity::Horizontal => child.drawable.bounding_box().height(),
            };

            let (main_axis, cross_axis) = match align {
                Align::Start => (main_axis_pen, 0.),
                Align::Center => (main_axis_pen, (cross_axis_length - child_cross_axis_length) / 2.),
                Align::End => (main_axis_pen, cross_axis_length - child_cross_axis_length),
                Align::Baseline => (main_axis_pen, axis_params.baseline_pos() - child.drawable.bounding_box().baseline_pos()),
                Align::Axis => (main_axis_pen, axis_params.axis_pos() - child.drawable.bounding_box().axis_pos())
            };

            child.point = match self.gravity {
                Gravity::Vertical => Point::new(cross_axis, main_axis),
                Gravity::Horizontal => Point::new(main_axis, cross_axis),
            };

            main_axis_pen += match self.gravity {
                Gravity::Horizontal => child.drawable.bounding_box().width(),
                Gravity::Vertical => child.drawable.bounding_box().height(),
            }
        }

        let (width, height) = match self.gravity {
            Gravity::Horizontal => (main_axis_pen, cross_axis_length),
            Gravity::Vertical => (cross_axis_length, main_axis_pen),
        };

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: cross_axis_length - axis_params.baseline_pos(),
            axis: cross_axis_length - axis_params.axis_pos(),
        };
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl<'a> LinearLayout<'a> {
    pub fn new() -> LinearLayout<'a> {
        LinearLayout {
            children: Vec::new(),
            gravity: Gravity::Horizontal,
            layout_align: Align::Center,
            bounding_box: BoundingBox::default(),
        }
    }

    fn get_align_self<'b>(parent_gravity: &Gravity, parent_align: &'b Align,
                          child_align: Option<&'b Align>) -> &'b Align {
        let alignment = if let Some(ref child) = child_align {
            child
        } else {
            parent_align
        };

        if *parent_gravity == Gravity::Vertical && (*alignment == Align::Axis || *alignment == Align::Baseline) {
            panic!("Vertical layout cannot be aligned to baseline / axis")
        }

        alignment
    }

    pub fn add_child(&mut self, drawable: Box<Drawable + 'a>, params: LinearLayoutParams) {
        self.children.push(Child { drawable, params, point: Point::new(0., 0.) })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;
    use ::platform::test::test_context;

    #[test]
    fn it_aligns_to_baseline() {
        let test_context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&test_context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 20., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 20.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 30., 10., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 30.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 10.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));

        ll.add_child(Box::new(Fixed::new(20., 40., 35., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 50.);
        assert_eq!(ll.bounding_box().height(), 55.);
        assert_eq!(ll.bounding_box().baseline(), 35.);
        assert_eq!(ll.children[0].point, Point::new(0., 10.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));
        assert_eq!(ll.children[2].point, Point::new(30., 15.));
    }

    #[test]
    fn it_aligns_to_axis() {
        let test_context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Axis;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&test_context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 20., 0., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 20.);
        assert_eq!(ll.bounding_box().axis(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 30., 0., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 30.);
        assert_eq!(ll.bounding_box().axis(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 10.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));

        ll.add_child(Box::new(Fixed::new(20., 40., 0., 35.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 50.);
        assert_eq!(ll.bounding_box().height(), 55.);
        assert_eq!(ll.bounding_box().axis(), 35.);
        assert_eq!(ll.children[0].point, Point::new(0., 10.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));
        assert_eq!(ll.children[2].point, Point::new(30., 15.));
    }

    #[test]
    fn it_aligns_to_top() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Start;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 20., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 20.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 30., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 30.);
        assert_eq!(ll.bounding_box().baseline(), 20.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));
    }

    #[test]
    fn it_aligns_to_center() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Center;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 20., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 20.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 30., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 30.);
        assert_eq!(ll.bounding_box().baseline(), 15.);
        assert_eq!(ll.children[0].point, Point::new(0., 5.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));
    }

    #[test]
    fn it_aligns_to_bottom() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::End;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 30., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 20.));
    }

    #[test]
    fn align_self_works() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 30., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 60.);
        assert_eq!(ll.bounding_box().baseline(), 20.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 30.));

        ll.add_child(Box::new(Fixed::new(20., 40., 20., 15.)),
                     LinearLayoutParams::new().with_align(Some(Align::Center)));
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 50.);
        assert_eq!(ll.bounding_box().height(), 60.);
        assert_eq!(ll.bounding_box().baseline(), 20.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 30.));
        assert_eq!(ll.children[2].point, Point::new(30., 10.));
    }

    #[test]
    fn align_self_works_when_basis_child_is_aligned_to_top() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new().with_align(Some(Align::Start)));
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 20., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 60.);
        assert_eq!(ll.bounding_box().baseline(), 20.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 40.));

        ll.add_child(Box::new(Fixed::new(20., 60., 0., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 50.);
        assert_eq!(ll.bounding_box().height(), 60.);
        assert_eq!(ll.bounding_box().baseline(), 20.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 40.));
        assert_eq!(ll.children[2].point, Point::new(30., -20.));
    }

    #[test]
    fn align_self_works_when_basis_child_is_aligned_to_center() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new().with_align(Some(Align::Center)));
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 20., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 40.));

        ll.add_child(Box::new(Fixed::new(20., 60., 10., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 50.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 40.));
        assert_eq!(ll.children[2].point, Point::new(30., -10.));

        ll.add_child(Box::new(Fixed::new(20., 80., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 70.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 40.));
        assert_eq!(ll.children[2].point, Point::new(30., -10.));
        assert_eq!(ll.children[3].point, Point::new(50., -20.));
    }

    #[test]
    fn align_self_works_when_basis_child_is_aligned_to_bottom() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new().with_align(Some(Align::End)));
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 10.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 20., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 40.));

        ll.add_child(Box::new(Fixed::new(20., 100., 20., 15.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 50.);
        assert_eq!(ll.bounding_box().height(), 90.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 40.));
        assert_eq!(ll.children[1].point, Point::new(10., 80.));
        assert_eq!(ll.children[2].point, Point::new(30., 0.));
    }

    #[test]
    fn first_non_flex_child_should_be_used_for_axis_and_baseline() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new().with_cross_axis_bound_mode(CrossAxisBoundMode::FillParent));

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new().with_cross_axis_bound_mode(CrossAxisBoundMode::FillParent));

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 20.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 0.);
        assert_eq!(ll.bounding_box().axis(), 0.);

        ll.add_child(Box::new(Fixed::new(10., 50., 20., 30.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 20.);
        assert_eq!(ll.bounding_box().axis(), 30.);
    }

    #[test]
    fn it_can_flex_to_cross_axis() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new());

        ll.add_child(
            Box::new(Fixed::new(10., 20., 10., 10.)),
            LinearLayoutParams::new().with_cross_axis_bound_mode(CrossAxisBoundMode::FillParent)
        );

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 20.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.bounding_box().axis(), 10.);

        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 0.));
        assert_eq!(ll.children[1].drawable.bounding_box().height(), 50.);
        assert_eq!(ll.children[0].drawable.bounding_box().height(), 50.);
    }

    #[test]
    fn it_aligns_vertical_layout_to_center() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.gravity = Gravity::Vertical;
        ll.layout_align = Align::Center;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(30., 50., 10., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.bounding_box().axis(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(10., 20., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 70.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(10., 50.));

    }

    #[test]
    fn it_aligns_vertical_layout_to_left() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.gravity = Gravity::Vertical;
        ll.layout_align = Align::Start;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(30., 50., 10., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.bounding_box().axis(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(10., 20., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 70.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(0., 50.));

    }

    #[test]
    fn it_aligns_vertical_layout_to_right() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.gravity = Gravity::Vertical;
        ll.layout_align = Align::End;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(30., 50., 10., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.bounding_box().axis(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(10., 20., 10., 10.)),
                     LinearLayoutParams::new());
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 30.);
        assert_eq!(ll.bounding_box().height(), 70.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(20., 50.));

    }

    #[test]
    fn vertical_layout_align_self_works() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.gravity = Gravity::Vertical;
        ll.layout_align = Align::Center;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(100., 50., 10., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 100.);
        assert_eq!(ll.bounding_box().height(), 50.);
        assert_eq!(ll.bounding_box().baseline(), 10.);
        assert_eq!(ll.bounding_box().axis(), 10.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));

        ll.add_child(Box::new(Fixed::new(20., 20., 10., 10.)),
                     LinearLayoutParams::new().with_align(Some(Align::Start)));

        ll.add_child(Box::new(Fixed::new(20., 20., 10., 10.)),
                     LinearLayoutParams::new().with_align(Some(Align::End)));
        calculate(&mut ll);

        assert_eq!(ll.bounding_box().width(), 100.);
        assert_eq!(ll.bounding_box().height(), 90.);
        assert_eq!(ll.children[0].point, Point::new(0., 0.));
        assert_eq!(ll.children[1].point, Point::new(0., 50.));
        assert_eq!(ll.children[2].point, Point::new(80., 70.));

    }

    #[test]
    fn it_stretch_items_in_main_axis() {
        let context = test_context();
        let mut ll = LinearLayout::new();
        ll.gravity = Gravity::Vertical;
        ll.layout_align = Align::Center;

        let calculate = |ll: &mut LinearLayout| {
            ll.calculate(&context, &MeasureMode::Wrap, &MeasureMode::UpTo(200.));
        };

        ll.add_child(Box::new(Fixed::new(30., 20., 10., 10.)),
                     LinearLayoutParams::new());
        ll.add_child(Box::new(Fixed::new(100., 0., 10., 10.)),
                     LinearLayoutParams::new().with_weight(1.));
        ll.add_child(Box::new(Fixed::new(20., 0., 10., 10.)),
                     LinearLayoutParams::new().with_weight(2.));
        ll.add_child(Box::new(Fixed::new(30., 0., 10., 10.)),
                     LinearLayoutParams::new().with_weight(3.));
        ll.add_child(Box::new(Fixed::new(50., 60., 10., 10.)),
                     LinearLayoutParams::new());

        calculate(&mut ll);

        assert_eq!(ll.bounding_box().height(), 200.);
        assert_eq!(ll.bounding_box().width(), 100.);
        assert_eq!(ll.children[0].drawable.bounding_box().height(), 20.);
        assert_eq!(ll.children[1].drawable.bounding_box().height(), 20.);
        assert_eq!(ll.children[2].drawable.bounding_box().height(), 40.);
        assert_eq!(ll.children[3].drawable.bounding_box().height(), 60.);
        assert_eq!(ll.children[4].drawable.bounding_box().height(), 60.);

    }
}