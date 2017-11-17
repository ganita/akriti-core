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

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyModifier<T: Add<T, Output=T>+Sub<Output=T>+Clone> {
    Increment(T),
    Decrement(T),
    Set(T),
    NoChange
}

impl<T: Add<Output=T>+Sub<Output=T>+Clone> PropertyModifier<T> {
    pub fn value(&self, val: T) -> T {
        match *self {
            PropertyModifier::Increment(ref p) => val+p.clone(),
            PropertyModifier::Decrement(ref p) => val-p.clone(),
            PropertyModifier::Set(ref p) => p.clone(),
            PropertyModifier::NoChange => val
        }
    }
}