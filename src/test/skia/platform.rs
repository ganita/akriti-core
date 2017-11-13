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

use std::any::Any;

use ::paint::{TextRuler, MathRuler};
use super::ruler::Ruler as SnapshotRuler;
use super::canvas::Canvas;

pub struct Platform {
    ruler: SnapshotRuler,
}

impl Platform {
    pub fn new(typeface: &str) -> Platform {
        Platform { ruler: SnapshotRuler::new(typeface, 0) }
    }

    pub fn new_canvas(&self, width: f32, height: f32) -> Canvas {
        Canvas::new(width.max(1.), height.max(1.),
                    self.ruler.get_sk_typeface())
    }
}

impl ::platform::Platform for Platform {
    fn get_text_ruler(&self, size: f32) -> &TextRuler {
        self.ruler.set_size(size);
        &self.ruler
    }

    fn get_math_ruler(&self, size: f32) -> &MathRuler {
        self.ruler.set_size(size);
        &self.ruler
    }

    fn px_to_du(&self, px: f32) -> f32 {
        px
    }

    fn sp_to_du(&self, sp: f32) -> f32 {
        64.*sp
    }

    fn dp_to_du(&self, dp: f32) -> f32 {
        64.*dp
    }

    fn as_any(&self) -> &Any {
        self
    }
}