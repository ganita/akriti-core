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


use std::ops::{Add, Sub};
use super::Length;
use ::platform::Context;

#[derive(Clone, PartialEq, Debug)]
pub enum PseudoLength {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    DU(f32),

    PercentWidth(f32),
    PercentHeight(f32),
    PercentDepth(f32),

    Width(f32),
    Height(f32),
    Depth(f32),

    _Add(Box<PseudoLength>, Box<PseudoLength>),
    _Sub(Box<PseudoLength>, Box<PseudoLength>)
}

impl PseudoLength {
    pub fn get_length_du(&self, context: &Context, font_size_du: f32,
                         child_width: f32, child_height: f32, child_depth: f32) -> f32 {
        match *self {
            PseudoLength::PX(px) => Length::PX(px).get_length_du(context, font_size_du),
            PseudoLength::DP(dp) => Length::DP(dp).get_length_du(context, font_size_du),
            PseudoLength::SP(sp) => Length::SP(sp).get_length_du(context, font_size_du),
            PseudoLength::EM(em) => Length::EM(em).get_length_du(context, font_size_du),
            PseudoLength::DU(du) => Length::DU(du).get_length_du(context, font_size_du),

            PseudoLength::PercentWidth(val) => child_width*val/100.0,
            PseudoLength::PercentHeight(val) => child_height*val/100.0,
            PseudoLength::PercentDepth(val) => child_depth*val/100.0,

            PseudoLength::Width(val) => val*child_width,
            PseudoLength::Height(val) => val*child_height,
            PseudoLength::Depth(val) => val*child_depth,

            PseudoLength::_Add(ref v1, ref v2) =>
                v1.get_length_du(context, font_size_du, child_width, child_height, child_depth) +
                v2.get_length_du(context, font_size_du, child_width, child_height, child_depth),
            PseudoLength::_Sub(ref v1, ref v2) =>
                v1.get_length_du(context, font_size_du, child_width, child_height, child_depth) -
                v2.get_length_du(context, font_size_du, child_width, child_height, child_depth)
        }
    }
}

impl Add for PseudoLength {
    type Output = PseudoLength;

    fn add(self, rhs: PseudoLength) -> Self::Output {
        PseudoLength::_Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for PseudoLength {
    type Output = PseudoLength;

    fn sub(self, rhs: PseudoLength) -> Self::Output {
        PseudoLength::_Sub(Box::new(self), Box::new(rhs))
    }
}