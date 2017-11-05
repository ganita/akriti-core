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

#[test]
fn test_symbol() {
    struct Test;

    impl Element for Test {
        fn layout<'a>(&'a self, _: &Context) -> Box<Drawable + 'a> {
            unimplemented!()
        }
    }

    let test_element = Test {};

    let mut symbol = Symbol::new(
        &test_element,
        |_| '√' as u32,
        |_| true,
        |_| 64.,
        |_| f32::INFINITY,
        |_| 64.,
        |_| &Directionality::LTR,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut symbol, -1., &MeasureMode::Wrap,
                  1000., &MeasureMode::UpTo, "symbol_sqrt");

    let mut symbol = Symbol::new(
        &test_element,
        |_| '←' as u32,
        |_| true,
        |_| 64.,
        |_| f32::INFINITY,
        |_| 64.,
        |_| &Directionality::LTR,
        |_| &Color::RGB(0, 0, 0)
    );

    snap_drawable(&mut symbol, 1000., &MeasureMode::UpTo,
                  -1., &MeasureMode::Wrap, "symbol_left_arrow");

}