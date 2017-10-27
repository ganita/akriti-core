use std::f32;

use super::{Drawable, BoundingBox, MeasureMode};
use ::platform::Context;
use ::paint::{Canvas, Point, Rect};

pub struct LinearLayout {
    children: Vec<Child>,
    pub gravity: Gravity,
    pub layout_align: Align,
    pub weight_sum: f32,

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

    pub fn new_aligned(align: Align) -> LinearLayoutParams {
        LinearLayoutParams {
            align_self: Some(align),
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

pub struct Child {
    drawable: Box<Drawable>,
    params: LinearLayoutParams,
    point: Point
}

impl Drawable for LinearLayout {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        for child in self.children.iter() {
            child.drawable.draw(canvas, &(pen_pos+&child.point));
        }
    }

    fn calculate(&mut self, context: &Context, _: f32, _: &MeasureMode, _: f32,
                 _: &MeasureMode) {
        if self.children.len() == 0 {
            self.bounding_box = BoundingBox::default();
            return ();
        }

        if self.children.len() == 1 {
            self.children[0].drawable.calculate(context, -1., &MeasureMode::Wrap, -1.,
                                                &MeasureMode::Wrap);
            self.bounding_box = self.children[0].drawable.bounding_box().clone();
            return ();
        }

        let layout_align = &self.layout_align;
        let layout_gravity = &self.gravity;
        let mut basis_child_align: Align = Align::Start;

        let mut baseline_pos: Option<f32> = None;
        let mut axis_pos: Option<f32> = None;
        let mut bottom_pos = 0f32;

        for child in self.children.iter_mut() {
            child.drawable.calculate(context, -1., &MeasureMode::Wrap,
                                     -1., &MeasureMode::Wrap);

            let align = LinearLayout::get_align_self(layout_gravity,
                                                     layout_align,
                                                     child.params.align_self.as_ref());

            bottom_pos = match *align {
                Align::Start | Align::End | Align::Center => {
                    let local_bottom_pos = bottom_pos.max(child.drawable.bounding_box().height());
                    if let Some(pos) = baseline_pos {
                        baseline_pos = Some(match basis_child_align {
                            Align::Start => pos,
                            Align::Center | Align::Baseline | Align::Axis => ((local_bottom_pos-bottom_pos)/2.) + pos,
                            Align::End => local_bottom_pos-(bottom_pos-pos),
                        });

                    } else if child.params.cross_axis_bound_mode != CrossAxisBoundMode::FillParent {
                        basis_child_align = align.clone();
                        baseline_pos = Some(child.drawable.bounding_box().baseline_pos());
                        axis_pos = Some(child.drawable.bounding_box().axis_pos());
                    }

                    local_bottom_pos
                },
                Align::Baseline => {
                    let (n_height, n_baseline_pos) = if let Some(pos) = baseline_pos {
                        let new_ascent = pos.max(child.drawable.bounding_box().baseline_pos());
                        let new_descent = child.drawable.bounding_box().baseline()
                            .max(bottom_pos-pos);

                        match basis_child_align {
                            Align::Start => (bottom_pos.max(pos+new_descent), pos),
                            Align::Center => (bottom_pos, pos),
                            Align::End => (new_ascent+bottom_pos-pos, new_ascent),
                            Align::Baseline => (new_ascent+new_descent, new_ascent),
                            Align::Axis => (bottom_pos.max(new_ascent+new_descent), new_ascent)
                        }
                    } else {
                        (
                            bottom_pos.max(child.drawable.bounding_box().height()),
                            child.drawable.bounding_box().baseline_pos()
                        )
                    };

                    if baseline_pos.is_none() && child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                        n_height
                    } else {
                        axis_pos = Some(if let Some(val) = baseline_pos{
                            axis_pos.unwrap()+n_baseline_pos-val
                        } else {
                            basis_child_align = align.clone();
                            n_baseline_pos+(child.drawable.bounding_box().axis()
                                -child.drawable.bounding_box().baseline())
                        });
                        baseline_pos = Some(n_baseline_pos);

                        n_height
                    }
                },
                Align::Axis => {
                    let (n_height, n_axis_pos) = if let Some(pos) = axis_pos {
                        let new_ascent = pos.max(child.drawable.bounding_box().axis_pos());
                        let new_descent = child.drawable.bounding_box().axis()
                            .max(bottom_pos-pos);

                        match basis_child_align {
                            Align::Start => (bottom_pos.max(pos+new_descent), pos),
                            Align::Center => (bottom_pos, pos),
                            Align::End => (new_ascent+bottom_pos-pos, new_ascent),
                            Align::Baseline => (new_ascent+new_descent, new_ascent),
                            Align::Axis => (bottom_pos.max(new_ascent+new_descent), new_ascent)
                        }
                    } else {
                        (bottom_pos.max(
                            child.drawable.bounding_box().height()
                        ), child.drawable.bounding_box().baseline_pos())
                    };

                    if axis_pos.is_none() && child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                        n_height
                    } else {
                        baseline_pos = Some(if let Some(val) = axis_pos {
                            baseline_pos.unwrap() + n_axis_pos - val
                        } else {
                            basis_child_align = align.clone();
                            n_axis_pos + (child.drawable.bounding_box().baseline()
                                - child.drawable.bounding_box().axis())
                        });

                        axis_pos = Some(n_axis_pos);

                        n_height
                    }
                }
            };
        }

        let height = bottom_pos;
        let baseline_pos = baseline_pos.unwrap_or(height);
        let axis_pos = axis_pos.unwrap_or(height);
        let mut width = 0f32;
        for child in self.children.iter_mut() {
            if child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                child.drawable.calculate(context, -1., &MeasureMode::Wrap,
                                            height, &MeasureMode::UpTo);
            }

            let align = LinearLayout::get_align_self(
                &self.gravity,
                &self.layout_align,
                child.params.align_self.as_ref()
            );

            let pos = match *align {
                Align::Start => Point::new(width, 0.),
                Align::Center => Point::new(width, (height-child.drawable.bounding_box().height())/2.),
                Align::End => Point::new(width, height-child.drawable.bounding_box().height()),
                Align::Baseline => Point::new(width, baseline_pos-child.drawable.bounding_box().baseline_pos()),
                Align::Axis => Point::new(width, axis_pos-child.drawable.bounding_box().axis_pos())
            };

            child.point = pos;

            width += child.drawable.bounding_box().width();
        }

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: height-baseline_pos,
            axis: height-axis_pos,
        };
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl LinearLayout {
    pub fn new() -> LinearLayout {
        LinearLayout {
            children: Vec::new(),
            gravity: Gravity::Horizontal,
            layout_align: Align::Center,
            weight_sum: 0.0,
            bounding_box: BoundingBox::default(),
        }
    }

    fn get_align_self<'a>(parent_gravity: &Gravity, parent_align: &'a Align,
                          child_align: Option<&'a Align>) -> &'a Align {
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

    pub fn add_child(&mut self, drawable: Box<Drawable>, params: LinearLayoutParams) {
        self.children.push(Child { drawable, params, point: Point::new(0., 0.) })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;
    use ::platform::{Platform, Context};
    use ::paint::{TextRuler, MathRuler};
    use ::elements::Element;

    pub struct MockPlatform{}
    impl Platform for MockPlatform {
        fn get_text_ruler(&self, _: &Element, _: f32) -> &TextRuler {
            unimplemented!()
        }

        fn get_math_ruler(&self, _: &Element, _: f32) -> &MathRuler {
            unimplemented!()
        }

        fn px_to_du(&self, _: f32) -> f32 {
            unimplemented!()
        }

        fn sp_to_du(&self, _: f32) -> f32 {
            unimplemented!()
        }

        fn dp_to_du(&self, _: f32) -> f32 {
            unimplemented!()
        }
    }

    fn test_context() -> Context {
        Context::new(Box::new(MockPlatform {}), 12.)
    }

    #[test]
    fn it_aligns_to_baseline() {
        let test_context = test_context();
        let mut ll = LinearLayout::new();
        ll.layout_align = Align::Baseline;

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&test_context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&test_context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
                     LinearLayoutParams::new_aligned(Align::Center));
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new_aligned(Align::Start));
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new_aligned(Align::Center));
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new_aligned(Align::End));
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

        let calculate = | ll: &mut LinearLayout| {
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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

    }
}