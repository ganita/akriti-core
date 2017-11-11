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


mod presentation;               pub use self::presentation::*;

mod token;                      pub use self::token::*;
mod general_layout;             pub use self::general_layout::*;

mod mempty;                     pub use self::mempty::*;

use std::any::Any;

use ::platform::Context;
use ::draw::Drawable;

pub trait Layout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a>;
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

pub trait ConcreteLayout<'a, T: Drawable + 'a> {
    fn layout(&'a self, context: &Context) -> T;
}

pub trait ElementGroup : Layout {
    fn children(&self) -> &[Box<Layout>];
}