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


extern crate akriti_core;
extern crate skia_sys as skia;
extern crate akriti_measure;

pub mod ruler;
pub mod canvas;
pub mod platform;

use std::path::Path;

pub struct Snapshot {
    platform: platform::Platform
}

impl Snapshot {
    pub fn new() -> Snapshot {
        unimplemented!()
    }

    pub fn load() -> Snapshot {
        unimplemented!()
    }

    pub fn save(&self, path: &Path) {
        unimplemented!()
    }

    pub fn diff(&self, other: &Snapshot) -> Snapshot {
        unimplemented!()
    }

    pub fn open(&self) {
        unimplemented!()
    }
}

impl PartialEq for Snapshot {
    fn eq(&self, other: &Snapshot) -> bool {
        unimplemented!()
    }
}