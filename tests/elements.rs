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

mod common;     use common::snap_element;

use std::f32;

use akriti_core::layout::*;
use akriti_core::props::*;

#[test]
fn test_mrow() {
    let mut mrow = MrowElement::new(Directionality::LTR, Color::RGB(0, 0, 0),
                                    Color::RGB(255, 255, 255));

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

    snap_element(&mrow, "mrow");
}

#[test]
fn test_mi() {
    let mi = MiElement::new(
        String::from("Hello world"),
        MathVariant::Normal,
        64.,
        Directionality::LTR,
        Color::RGB(0, 0, 0),
        Color::transparent()
    );

    snap_element(&mi, "mi");
}

#[test]
fn test_mo() {
    let mut mrow = MrowElement::new(Directionality::LTR, Color::RGB(0, 0, 0),
                                    Color::RGB(255, 255, 255));

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
        Box::new(MoElement::new(
            String::from("+"),
            MathVariant::Normal,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            12.,
            12.,
            false,
            false,
            f32::INFINITY,
            64.,
            false,
            false,
            false,
            LineBreak::Auto,
            20.,
            LineBreakStyle::After,
            String::from(""),
            IndentAlign::Auto,
            20.,
            None,
            IndentAlignFirstLast::Auto,
            IndentShiftFirstLast::IndentShift,
            IndentAlignFirstLast::Auto,
            IndentShiftFirstLast::IndentShift,
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

    snap_element(&mrow, "mo");
}


#[test]
fn test_mfrac() {
    let numerator = MiElement::new(
        String::from("x"),
        MathVariant::Italic,
        64.,
        Directionality::LTR,
        Color::RGB(0, 0, 0),
        Color::transparent()
    );

    let denominator = MiElement::new(
        String::from("y"),
        MathVariant::Italic,
        64.,
        Directionality::LTR,
        Color::RGB(0, 0, 0),
        Color::transparent()
    );

    let mfrac = MfracElement::new(
        Box::new(numerator),
        Box::new(denominator),
        2.,
        HAlign::Center,
        HAlign::Center,
        false,
        Directionality::LTR,
        Color::RGB(0, 0, 0),
        Color::transparent()
    );

    snap_element(&mfrac, "mfrac");
}