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
pub enum VAlign {
    Top,
    Bottom,
    Center,
    Baseline,
    Axis
}

#[derive(Debug, Clone)]
pub struct TableVAlign {
    align: VAlign,
    row_number: Option<i32>
}

impl TableVAlign {
    pub fn new(align: VAlign, row_number: Option<i32>) -> TableVAlign {
        TableVAlign {
            align,
            row_number
        }
    }

    pub fn align(&self) -> &VAlign {
        &self.align
    }

    pub fn row_number(&self) -> &Option<i32> {
        &self.row_number
    }
}