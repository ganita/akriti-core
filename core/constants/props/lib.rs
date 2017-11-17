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

extern crate phf_shared;
use std::hash::{Hasher, Hash};

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum OperatorForm {
    Infix = 0,
    Prefix = 1,
    Postfix = 2
}

impl phf_shared::PhfHash for OperatorForm {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        let int_value = *self as u8;
        int_value.hash(state);
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum MathVariant {
    Normal = 0,
    Bold = 1,
    Italic = 2,
    BoldItalic = 3,
    DoubleStruck = 4,
    BoldFraktur = 5,
    Script = 6,
    BoldScript = 7,
    Fraktur = 8,
    SansSerif = 9,
    BoldSansSerif = 10,
    SansSerifItalic = 11,
    SansSerifBoldItalic = 12,
    Monospace = 13,
    Initial = 14,
    Tailed = 15,
    Looped = 16,
    Stretched = 17,
}

impl phf_shared::PhfHash for MathVariant {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        let int_value = *self as u8;
        int_value.hash(state);
    }
}
