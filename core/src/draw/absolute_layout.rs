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


use super::{Drawable, BoundingBox, MeasureMode};
use ::platform::Context;
use ::paint::{Canvas, Point, Rect};
use std::cmp::Ordering;
use std::slice::{Iter, IterMut};

pub struct AbsoluteLayout<'a> {
    children: Vec<Child<'a>>,
    bounding_box: BoundingBox,

    baseline: Option<f32>,
    axis: Option<f32>,

    calculate_child_bounds: bool,
}

pub struct Child<'a> {
    drawable: Box<Drawable+'a>,
    params: AbsoluteLayoutParams
}

impl<'a> Child<'a> {
    pub fn drawable(&self) -> &Drawable {
        self.drawable.as_ref()
    }

    pub fn params(&self) -> &AbsoluteLayoutParams {
        &self.params
    }
}

pub struct AbsoluteLayoutParams {
    position: Point
}

impl AbsoluteLayoutParams {
    pub fn new(point: Point) -> AbsoluteLayoutParams {
        AbsoluteLayoutParams { position: point }
    }
}

impl<'a> Drawable for AbsoluteLayout<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        for child in &self.children {
            child.drawable.draw(canvas, &(pen_pos+&child.params.position))
        }
    }

    fn calculate(&mut self, context: &Context, _: &MeasureMode, _: &MeasureMode) {
        if self.calculate_child_bounds {
            for child in self.children.iter_mut() {
                child.drawable.calculate(context, &MeasureMode::Wrap,&MeasureMode::Wrap);
            }
        }

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

impl<'a> AbsoluteLayout<'a> {
    pub fn new() -> AbsoluteLayout<'a> {
        AbsoluteLayout {
            children: Vec::new(),
            bounding_box: BoundingBox::default(),
            baseline: None,
            axis: None,
            calculate_child_bounds: true
        }
    }

    pub fn add_child(&mut self, child: Box<Drawable + 'a>, params: AbsoluteLayoutParams) {
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

    pub fn iter(&self) -> Iter<Child<'a>> {
        self.children.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Child<'a>> {
        self.children.iter_mut()
    }

    pub fn should_calculate_child_bounds(&mut self, calculate: bool) {
        self.calculate_child_bounds = calculate;
    }

    pub fn is_calculating_child_bounds(&self) -> bool {
        self.calculate_child_bounds
    }
}