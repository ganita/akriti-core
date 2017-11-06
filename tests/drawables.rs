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


extern crate akriti_core;

mod common;     use common::snap_drawable;

use std::f32;

use akriti_core::elements::*;
use akriti_core::draw::*;
use akriti_core::props::*;
use akriti_core::platform::Context;
use akriti_core::paint::*;


struct Test;

impl Element for Test {
    fn layout<'a>(&'a self, _: &Context) -> Box<Drawable + 'a> {
        unimplemented!()
    }
}

#[test]
fn test_symbol() {
    let test_element = Test {};

    let mut symbol = Symbol::new(
        &test_element,
        |_| "√",
        |_| &MathVariant::Normal,
        |_| true,
        |_| 64.,
        |_| f32::INFINITY,
        |_| 64.,
        |_| &Directionality::LTR,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut symbol, &MeasureMode::Wrap,
                  &MeasureMode::UpTo(1000.), "symbol_sqrt");

    let mut symbol = Symbol::new(
        &test_element,
        |_| "←",
        |_| &MathVariant::Normal,
        |_| true,
        |_| 64.,
        |_| f32::INFINITY,
        |_| 64.,
        |_| &Directionality::LTR,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut symbol, &MeasureMode::UpTo(1000.),
                  &MeasureMode::Wrap, "symbol_left_arrow");


    let mut symbol = Symbol::new(
        &test_element,
        |_| "+",
        |_| &MathVariant::Normal,
        |_| true,
        |_| 64.,
        |_| f32::INFINITY,
        |_| 64.,
        |_| &Directionality::LTR,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut symbol, &MeasureMode::UpTo(1000.),
                  &MeasureMode::Wrap, "symbol_plus");

    let mut symbol = Symbol::new(
        &test_element,
        |_| "hello",
        |_| &MathVariant::Normal,
        |_| true,
        |_| 64.,
        |_| f32::INFINITY,
        |_| 64.,
        |_| &Directionality::LTR,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut symbol, &MeasureMode::UpTo(1000.),
                  &MeasureMode::Wrap, "symbol_text");
}

#[test]
fn test_line() {
    let element = Test { };

    let mut line = Line::new(
        LineParam::Fixed { start: Point::new(0., 0.), end: Point::new(100., 100.) },
        &element,
        |_| 50.,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut line, &MeasureMode::Wrap, &MeasureMode::Wrap,
                  "line_45deg");

    let mut line = Line::new(
        LineParam::Fixed { start: Point::new(0., 0.), end: Point::new(50., 100.) },
        &element,
        |_| 50.,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut line, &MeasureMode::Wrap, &MeasureMode::Wrap,
                  "line_inclined");

    let mut line = Line::new(
        LineParam::Vertical { x: 0. },
        &element,
        |_| 50.,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut line, &MeasureMode::Wrap, &MeasureMode::UpTo(100.),
                  "line_vertical");

    let mut line = Line::new(
        LineParam::Horizontal { y: 0. },
        &element,
        |_| 50.,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut line, &MeasureMode::UpTo(100.), &MeasureMode::Wrap,
                  "line_horizontal");
}