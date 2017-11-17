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
pub enum MathSize {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    SMALL,
    NORMAL,
    BIG
}

const BIG_MATH_SIZE_MULTIPLIER: f32 = 1.2;
const SMALL_MATH_SIZE_MULTIPLIER: f32 = 0.8;

impl MathSize {
    pub fn get_math_size_du(&self, context: &Context, nominal_font_size_px: f32) -> f32 {
        match *self {
            MathSize::PX(px) => Length::PX(px).get_length_du(context, nominal_font_size_px),
            MathSize::DP(dp) => Length::DP(dp).get_length_du(context, nominal_font_size_px),
            MathSize::SP(sp) => Length::SP(sp).get_length_du(context, nominal_font_size_px),
            MathSize::EM(em) => Length::EM(em).get_length_du(context, nominal_font_size_px),
            MathSize::BIG => MathSize::NORMAL.get_math_size_du(context, nominal_font_size_px)
                *BIG_MATH_SIZE_MULTIPLIER,
            MathSize::NORMAL => nominal_font_size_px,
            MathSize::SMALL => MathSize::NORMAL.get_math_size_du(context, nominal_font_size_px)
                *SMALL_MATH_SIZE_MULTIPLIER,
        }
    }
}