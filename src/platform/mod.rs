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


mod context;                pub use self::context::Context;

use std::any::Any;

use ::paint::{
    TextRuler, 
    MathRuler
};
use ::layout::Layout;

pub trait Platform {
    fn get_text_ruler(&self, size: f32) -> &TextRuler;
    fn get_math_ruler(&self, size: f32) -> &MathRuler;
    fn px_to_du(&self, px: f32) -> f32;
    fn sp_to_du(&self, sp: f32) -> f32;
    fn dp_to_du(&self, dp: f32) -> f32;
    fn as_any(&self) -> &Any;
}

#[cfg(test)] pub mod test;