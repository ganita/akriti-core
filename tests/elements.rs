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
extern crate akriti_snapshot;

use std::path::Path;

use akriti_core::paint::{Point};
use akriti_core::elements::Element;
use akriti_core::elements::{MiElement, MrowElement};
use akriti_core::props::*;
use akriti_snapshot::platform::Platform;
use akriti_snapshot::canvas::Canvas;
use akriti_core::platform::Context;
use akriti_core::paint::Canvas as AkritiCanvas;


#[test]
fn it_works() {
    let mut mrow = MrowElement::new(Directionality::LTR, Color::RGB(255, 255, 255));

    mrow.add_element(
        Box::new(MiElement::new(
            String::from("a"),
            MathVariant::Italic,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::transparent()
        ))
    );

    mrow.add_element(
        Box::new(MiElement::new(
            String::from(" + "),
            MathVariant::Normal,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::transparent()
        ))
    );

    mrow.add_element(
        Box::new(MiElement::new(
            String::from("x"),
            MathVariant::Italic,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::transparent()
        ))
    );

    mrow.add_element(
        Box::new(MiElement::new(
            String::from("i"),
            MathVariant::Italic,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::transparent()
        ))
    );

    let root_dir = env!("CARGO_MANIFEST_DIR");
    let font = format!("{}/tests/fonts/STIX2Math.otf", root_dir);

    let context = Context::new(Box::new(Platform::new(&font)), 64.);

    let layout = mrow.layout(&context);

    let canvas: Canvas = context.platform().as_any().downcast_ref::<Platform>().unwrap()
        .new_canvas(layout.bounding_box().width(), layout.bounding_box().height());

    layout.draw(&canvas, &Point::new(0., 0.));

    canvas.as_any().downcast_ref::<Canvas>().unwrap().snapshot(Path::new("target/mi.png"));
}
