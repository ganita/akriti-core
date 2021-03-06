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

use super::{Layout};
use ::platform::Context;
use ::draw::{Drawable, Empty};

pub struct MemptyLayout {}

impl MemptyLayout {
    pub fn new() -> MemptyLayout {
        MemptyLayout {}
    }
}

impl Layout for MemptyLayout {
    fn layout<'a>(&'a self, _: &Context) -> Box<Drawable + 'a> {
        Box::new(Empty::new())
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}