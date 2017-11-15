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


mod accent;                 pub use self::accent::*;
mod color;                  pub use self::color::*;
mod column_width;           pub use self::column_width::*;
mod directionality;         pub use self::directionality::*;
mod frame_spacing;          pub use self::frame_spacing::*;
mod group_align;            pub use self::group_align::*;
mod halign;                 pub use self::halign::*;
mod id_ref;                 pub use self::id_ref::*;
mod indent;                 pub use self::indent::*;
mod length;                 pub use self::length::*;
mod linebreak;              pub use self::linebreak::*;
mod line_thickness;         pub use self::line_thickness::*;
mod line_type;              pub use self::line_type::*;
mod mathsize;               pub use self::mathsize::*;
mod mathvariant;            pub use self::mathvariant::*;
mod notation;               pub use self::notation::*;
mod operator_form;          pub use self::operator_form::*;
mod property_modifier;      pub use self::property_modifier::*;
mod pseudo_length;          pub use self::pseudo_length::*;
mod script_level;           pub use self::script_level::*;
mod valign;                 pub use self::valign::*;
mod table_side;             pub use self::table_side::*;

pub type DisplayStyle = bool;
pub type ScriptMinSize = f32;
pub type ScriptSizeMultiplier = f32;
