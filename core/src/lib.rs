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


#[macro_use] extern crate akriti_macros;
#[macro_use] extern crate lazy_static;

pub extern crate akriti_constants;
pub use akriti_constants as constants;

pub mod draw;
pub mod elements;
pub mod layout;
pub mod paint;
pub mod props;
pub mod platform;
pub mod utils;

#[cfg(test)] pub extern crate akriti_measure;
#[cfg(test)] pub extern crate skia_sys;
#[cfg(test)] pub mod test;