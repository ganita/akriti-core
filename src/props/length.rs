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
use ::constants::SpaceLevel;

#[derive(Clone, PartialEq, Debug)]
pub enum Length {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    EX(f32),
    SpaceLevel(SpaceLevel),
    Infinity,
    Auto
}

impl Length {
    pub fn get_length_du(&self, context: &Context, font_size_du: f32) -> f32 {
        match *self {
            Length::PX(px) => context.platform().px_to_du(px),
            Length::DP(dp) => context.platform().dp_to_du(dp),
            Length::SP(sp) => context.platform().sp_to_du(sp),
            Length::EM(em) => em*font_size_du,
            Length::SpaceLevel(ref level) => level.em()*font_size_du,
            Length::Infinity => f32::INFINITY,
            Length::Auto => f32::NAN,
            Length::EX(_) => unimplemented!()
        }
    }
}