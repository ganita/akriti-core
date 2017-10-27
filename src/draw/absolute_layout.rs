use super::{Drawable, BoundingBox, MeasureMode};
use ::platform::Context;
use ::paint::{Canvas, Point, Rect};
use std::cmp::Ordering;
use std::slice::{Iter, IterMut};

pub struct AbsoluteLayout {
    children: Vec<Child>,
    bounding_box: BoundingBox,

    baseline: Option<f32>,
    axis: Option<f32>,
}

pub struct Child {
    drawable: Box<Drawable>,
    params: AbsoluteLayoutParams
}

pub struct AbsoluteLayoutParams {
    position: Point
}

impl Drawable for AbsoluteLayout {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        for child in &self.children {
            child.drawable.draw(canvas, &(pen_pos+&child.params.position))
        }
    }

    fn calculate(&mut self, _: &Context, _: f32, _: &MeasureMode, _: f32,
                 _: &MeasureMode) {
        let end_x_calc = |child: &Child|
            child.drawable.bounding_box().width() + child.params.position.x();

        let width = self.children.iter().max_by(|c1, c2| {
            end_x_calc(*c1).partial_cmp(&end_x_calc(*c2)).unwrap_or(Ordering::Less)
        }).and_then(|c| {
            Some(end_x_calc(c))
        }).unwrap_or(0f32);

        let end_y_calc = |child: &Child|
            child.drawable.bounding_box().height() + child.params.position.y();

        let height = self.children.iter().max_by(|c1, c2| {
            end_y_calc(*c1).partial_cmp(&end_y_calc(*c2)).unwrap_or(Ordering::Less)
        }).and_then(|c| {
            Some(end_y_calc(c))
        }).unwrap_or(0f32);

        self.bounding_box = BoundingBox {
            rect: Rect::new(width, height),
            baseline: self.baseline.unwrap_or(height),
            axis: self.axis.unwrap_or(height/2f32),
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl AbsoluteLayout {
    pub fn new() -> AbsoluteLayout {
        AbsoluteLayout {
            children: Vec::new(),
            bounding_box: BoundingBox::default(),
            baseline: None,
            axis: None,
        }
    }

    pub fn add_child(&mut self, child: Box<Drawable>, params: AbsoluteLayoutParams) {
        self.children.push(Child { drawable: child, params });
    }

    pub fn remove_child_at(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn clear(&mut self) {
        self.children.clear();
    }

    pub fn set_baseline(&mut self, baseline: Option<f32>) {
        self.baseline = baseline;
    }

    pub fn set_axis(&mut self, axis: Option<f32>) {
        self.axis = axis;
    }

    pub fn set_child_params(&mut self, index: usize, params: AbsoluteLayoutParams) {
        if let Some(val) = self.children.get_mut(index) {
            val.params = params;
        }
    }

    pub fn iter(&self) -> Iter<Child> {
        self.children.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Child> {
        self.children.iter_mut()
    }
}