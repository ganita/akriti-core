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
use ::platform::Context;
use super::length::Length;

#[derive(Clone, PartialEq, Debug)]
pub enum LineThickness {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    THIN,
    THICK,
    MEDIUM
}

const THICK_LINE_THICKNESS_MULTIPLIER: f32 = 1.2;
const THIN_LINE_THICKNESS_MULTIPLIER: f32 = 0.8;

impl LineThickness {
    pub fn get_thickness_du(&self, context: &Context, font_size_du: f32, nominal_rule_thickness: f32) -> f32 {
        match *self {
            LineThickness::PX(px) => Length::PX(px).get_length_du(context, font_size_du),
            LineThickness::DP(dp) => Length::DP(dp).get_length_du(context, font_size_du),
            LineThickness::SP(sp) => Length::SP(sp).get_length_du(context, font_size_du),
            LineThickness::EM(em) => Length::EM(em).get_length_du(context, font_size_du),
            LineThickness::THICK => LineThickness::MEDIUM.get_thickness_du(context, font_size_du, nominal_rule_thickness)
                *THICK_LINE_THICKNESS_MULTIPLIER,
            LineThickness::MEDIUM => nominal_rule_thickness,
            LineThickness::THIN => LineThickness::MEDIUM.get_thickness_du(context, font_size_du, nominal_rule_thickness)
                *THIN_LINE_THICKNESS_MULTIPLIER,
        }
    }
}