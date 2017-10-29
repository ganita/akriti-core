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
        let mut bottom_pos = 0f32;

        // In first iteration we will compute the dimension of children to just wrap the content
        // and compute the total size of container
        for child in self.children.iter_mut() {
            // Compute minimum dimension required by child to wrap its contents
            child.drawable.calculate(context, -1., &MeasureMode::Wrap,
                                     -1., &MeasureMode::Wrap);

            let align = LinearLayout::get_align_self(layout_gravity,
                                                     layout_align,
                                                     child.params.align_self.as_ref());

            bottom_pos = match *align {
                Align::Start | Align::End | Align::Center => {
                    let local_bottom_pos = bottom_pos.max(child.drawable.bounding_box().height());
                    if let Some(ref mut params) = axis_params {
                        params.baseline_pos = match params.align {
                            Align::Start => params.baseline_pos,

                            Align::Center | Align::Baseline | Align::Axis =>
                                ((local_bottom_pos - bottom_pos) / 2.) + params.baseline_pos,

                            Align::End => local_bottom_pos - (bottom_pos - params.baseline_pos),
                        };
                    }

                    local_bottom_pos
                }
                Align::Baseline | Align::Axis => {
                    let (n_height, n_axis_pos) = if let Some(ref mut params) = axis_params {
                        let new_ascent = if *align == Align::Baseline {
                            params.baseline_pos().max(child.drawable.bounding_box().baseline_pos())
                        } else {
                            params.axis_pos().max(child.drawable.bounding_box().axis_pos())
                        };

                        let new_descent = if *align == Align::Baseline {
                            child.drawable.bounding_box().baseline().max(bottom_pos - params.baseline_pos())
                        } else {
                            child.drawable.bounding_box().axis().max(bottom_pos - params.axis_pos())
                        };

                        let basis_axis_pos = if *align == Align::Baseline {
                            params.baseline_pos()
                        } else {
                            params.axis_pos()
                        };

                        match params.align {
                            Align::Start => (bottom_pos.max(basis_axis_pos + new_descent), basis_axis_pos),
                            Align::Center => (bottom_pos, basis_axis_pos),
                            Align::End => (new_ascent + bottom_pos - basis_axis_pos, new_ascent),
                            Align::Baseline => (new_ascent + new_descent, new_ascent),
                            Align::Axis => (bottom_pos.max(new_ascent + new_descent), new_ascent)
                        }
                    } else {
                        (
                            bottom_pos.max(child.drawable.bounding_box().height()),
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

                    n_height
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

        let height = bottom_pos;
        let axis_params = axis_params.unwrap_or(AxisParams {
            align: Align::Start,
            baseline_pos: height,
            axis_baseline_shift: 0.,
        });

        let mut width = 0f32;
        for child in self.children.iter_mut() {
            if child.params.cross_axis_bound_mode == CrossAxisBoundMode::FillParent {
                child.drawable.calculate(context, -1., &MeasureMode::Wrap,
                                         height, &MeasureMode::UpTo);
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

            let pos = match align {
                Align::Start => Point::new(width, 0.),
                Align::Center => Point::new(width, (height - child.drawable.bounding_box().height()) / 2.),
                Align::End => Point::new(width, height - child.drawable.bounding_box().height()),
                Align::Baseline => Point::new(width, axis_params.baseline_pos() - child.drawable.bounding_box().baseline_pos()),
                Align::Axis => Point::new(width, axis_params.axis_pos() - child.drawable.bounding_box().axis_pos())
            };

            child.point = pos;

            width += child.drawable.bounding_box().width();
        }

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: height - axis_params.baseline_pos(),
            axis: height - axis_params.axis_pos(),
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
}