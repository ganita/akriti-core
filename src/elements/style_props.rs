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

use ::props::*;

pub struct StyleProps {

}

impl StyleProps {
    pub fn math_background(&self) -> Option<&Color> {
        unimplemented!()
    }

    pub fn math_variant(&self) -> Option<&MathVariant> {
        unimplemented!()
    }

    pub fn ms_lquote(&self) -> Option<&String> { unimplemented!() }

    pub fn ms_rquote(&self) -> Option<&String> { unimplemented!() }

    pub fn mspace_width(&self) -> Option<&Length> { unimplemented!() }

    pub fn mspace_height(&self) -> Option<&Length> { unimplemented!() }

    pub fn mspace_depth(&self) -> Option<&Length> { unimplemented!() }

    pub fn linebreak(&self) -> Option<&LineBreak> { unimplemented!() }

    pub fn mo_form(&self) -> Option<&OperatorForm> { unimplemented!() }

    pub fn mo_fence(&self) -> Option<&bool> { unimplemented!() }

    pub fn mo_separator(&self) -> Option<&bool> { unimplemented!() }

    pub fn mo_lspace(&self) -> Option<&Length> { unimplemented!() }

    pub fn mo_rspace(&self) -> Option<&Length> { unimplemented!() }

    pub fn mo_stretchy(&self) -> Option<&bool> { unimplemented!() }

    pub fn mo_symmetric(&self) -> Option<&bool> { unimplemented!() }

    pub fn mo_max_size(&self) -> Option<&Length> { unimplemented!() }

    pub fn mo_min_size(&self) -> Option<&Length> { unimplemented!() }

    pub fn mo_large_op(&self) -> Option<&bool> { unimplemented!() }

    pub fn mo_movable_limits(&self) -> Option<&bool> { unimplemented!() }

    pub fn mo_accent(&self) -> Option<&bool> { unimplemented!() }
}