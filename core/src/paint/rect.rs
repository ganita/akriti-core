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


#[derive(Debug, Clone)]
pub struct Rect {
    width: f32,
    height: f32
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Rect {
        assert!(width >= 0.0, "Width of rectangle should be greater than zero");
        assert!(height >= 0.0, "Height of rectangle should be greater than zero");
        Rect {
            width,
            height
        }
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}