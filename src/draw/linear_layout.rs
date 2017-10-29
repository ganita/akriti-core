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

impl Drawable for LinearLayout {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        for child in self.children.iter() {
            child.drawable.draw(canvas, &(pen_pos + &child.point));
        }
    }

    fn calculate(&mut self, context: &Context, _: f32, _: &MeasureMode, _: f32, _: &MeasureMode) {
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

        let mut axis_params: Option<AxisParams> = None;
        let mut cross_axis_length = 0f32;

        // In first iteration we will compute the dimension of children to just wrap the content
        // and compute the total size of container
        for child in self.children.iter_mut() {
            // Compute minimum dimension required by child to wrap its contents
            child.drawable.calculate(context, -1., &MeasureMode::Wrap,
                                     -1., &MeasureMode::Wrap);

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

        let axis_params = axis_params.unwrap_or(AxisParams {
            align: Align::Start,
            baseline_pos: cross_axis_length,
            axis_baseline_shift: 0.,
        });

        let mut main_axis_pen = 0f32;
        for child in self.children.iter_mut() {
            if child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                let (width, width_mode, height, height_mode) = match self.gravity {
                    Gravity::Vertical => (cross_axis_length, MeasureMode::UpTo, -1., MeasureMode::Wrap),
                    Gravity::Horizontal => (-1., MeasureMode::Wrap, cross_axis_length, MeasureMode::UpTo),
                };

                child.drawable.calculate(context, width, &width_mode, height, &height_mode);
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

    pub struct MockPlatform {}

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

        let calculate = |ll: &mut LinearLayout| {
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

        let calculate = |ll: &mut LinearLayout| {
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

        let calculate = |ll: &mut LinearLayout| {
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

        let calculate = |ll: &mut LinearLayout| {
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

        let calculate = |ll: &mut LinearLayout| {
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

        let calculate = |ll: &mut LinearLayout| {
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
        };

        ll.add_child(Box::new(Fixed::new(10., 50., 10., 10.)),
                     LinearLayoutParams::new());

        let mut flex_child = Box::new(Fixed::new(10., 20., 10., 10.));
        flex_child.flex = true;
        ll.add_child(
            flex_child,
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
            ll.calculate(&context, -1., &MeasureMode::Wrap, -1.,
                         &MeasureMode::Wrap);
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
}