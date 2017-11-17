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


use super::Platform;
use ::constants::{
    OperatorDictionary, 
    MathVariantsDictionary
};

pub struct Context {
    platform: Box<Platform>,
    operator_dictionary: OperatorDictionary,
    math_variants_dictionary: MathVariantsDictionary,
    font_size: f32
}

impl Context {
    pub fn new(platform: Box<Platform>, font_size: f32) -> Context {
        Context {
            platform,
            operator_dictionary: OperatorDictionary::new(),
            math_variants_dictionary: MathVariantsDictionary::new(),
            font_size
        }
    }

    pub fn platform(&self) -> &Platform {
        self.platform.as_ref()
    }

    pub fn operator_dictionary(&self) -> &OperatorDictionary {
        &self.operator_dictionary
    }

    pub fn math_variants_dictionary(&self) -> &MathVariantsDictionary {
        &self.math_variants_dictionary
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }
}