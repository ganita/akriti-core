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


use super::{PropertyModifier, ScriptSizeMultiplier, ScriptMinSize, MathSize};
use ::platform::Context;

#[derive(Debug, Clone)]
pub struct ScriptLevel {
    level: u32,
    current_font_size: f32,
}

impl ScriptLevel {
    pub fn new(script_level: u32, current_font_size: f32) -> ScriptLevel {
        ScriptLevel {
            level: script_level,
            current_font_size,
        }
    }

    pub fn new_level(
        &self, modifier: PropertyModifier<i32>, context: &Context, math_size: &MathSize,
        script_size_multiplier: ScriptSizeMultiplier, script_min_size: ScriptMinSize
    ) -> ScriptLevel {
        let new_level = modifier.value(self.level as i32).max(0);

        let diff = new_level - (self.level as i32);
        let current_size = self.get_font_size(context, math_size);
        let new_size =
            (current_size * script_size_multiplier.powi(diff)).max(script_min_size);

        ScriptLevel {
            level: new_level as u32,
            current_font_size: new_size
        }
    }

    pub fn get_current_level(&self) -> u32 {
        self.level
    }

    pub fn get_font_size(&self, context: &Context, math_size: &MathSize) -> f32 {
        math_size.get_math_size_du(context, self.current_font_size)
    }
}