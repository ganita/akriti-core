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

use super::{Element, InstanceId, Miscellaneous, ElementType, InheritedProps, StyleProps, Family};
use ::layout::{MemptyLayout, Layout};
use ::platform::Context;

pub struct Mempty {
    instance_id: InstanceId
}

impl Mempty {
    pub fn new() -> Mempty {
        Mempty { instance_id: InstanceId::new() }
    }
}

impl Element for Mempty {
    fn layout<'a>(&self, _: &Context, _: &Family<'a>, _: &InheritedProps, _: &Option<&StyleProps>) -> Box<Layout> {
        Box::new(MemptyLayout::new())
    }

    fn type_info(&self) -> ElementType {
        ElementType::Miscellaneous(Miscellaneous::Mempty)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}