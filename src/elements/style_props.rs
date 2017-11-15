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

    pub fn mfrac_line_thickness(&self) -> Option<&LineThickness> { unimplemented!() }

    pub fn mfrac_num_align(&self) -> Option<&HAlign> { unimplemented!() }

    pub fn mfrac_denom_align(&self) -> Option<&HAlign> { unimplemented!() }

    pub fn mfrac_bevelled(&self) -> Option<&bool> { unimplemented!() }

    pub fn mfenced_open(&self) -> Option<&String> { unimplemented!() }

    pub fn mfenced_close(&self) -> Option<&String> { unimplemented!() }

    pub fn mfenced_separators(&self) -> Option<&String> { unimplemented!() }

    pub fn mpadded_width(&self) -> Option<&PropertyModifier<PseudoLength>> { unimplemented!() }

    pub fn mpadded_height(&self) -> Option<&PropertyModifier<PseudoLength>> { unimplemented!() }

    pub fn mpadded_depth(&self) -> Option<&PropertyModifier<PseudoLength>> { unimplemented!() }

    pub fn mpadded_lspace(&self) -> Option<&PropertyModifier<PseudoLength>> { unimplemented!() }

    pub fn mpadded_voffset(&self) -> Option<&PropertyModifier<PseudoLength>> { unimplemented!() }

    pub fn superscript_shift(&self) -> Option<&Length> { unimplemented!() }

    pub fn subscript_shift(&self) -> Option<&Length> { unimplemented!() }

    pub fn underover_accent(&self) -> Option<&Accent> { unimplemented!() }

    pub fn underover_accent_under(&self) -> Option<&Accent> { unimplemented!() }

    pub fn underover_align(&self) -> Option<&HAlign> { unimplemented!() }

    pub fn mtable_align(&self) -> Option<&TableVAlign> { unimplemented!() }

    pub fn mtable_row_align(&self) -> Option<&Vec<VAlign>> { unimplemented!() }

    pub fn mtable_column_align(&self) -> Option<&Vec<HAlign>> { unimplemented!() }

    pub fn mtable_group_align(&self) -> Option<&Vec<Vec<GroupAlign>>> { unimplemented!() }

    pub fn mtable_alignment_scope(&self) -> Option<&Vec<bool>> { unimplemented!() }

    pub fn mtable_column_width(&self) -> Option<&Vec<ColumnWidth>> { unimplemented!() }

    pub fn mtable_width(&self) -> Option<&Length> { unimplemented!() }

    pub fn mtable_row_spacing(&self) -> Option<&Vec<Length>> { unimplemented!() }

    pub fn mtable_column_spacing(&self) -> Option<&Vec<Length>> { unimplemented!() }

    pub fn mtable_row_lines(&self) -> Option<&Vec<LineType>> { unimplemented!() }

    pub fn mtable_column_lines(&self) -> Option<&Vec<LineType>> { unimplemented!() }

    pub fn mtable_frame(&self) -> Option<&LineType> { unimplemented!() }

    pub fn mtable_frame_spacing(&self) -> Option<&FrameSpacing> { unimplemented!() }

    pub fn mtable_equal_rows(&self) -> Option<&bool> { unimplemented!() }

    pub fn mtable_equal_columns(&self) -> Option<&bool> { unimplemented!() }

    pub fn mtable_display_style(&self) -> Option<&bool> { unimplemented!() }

    pub fn mtable_side(&self) -> Option<&TableSide> { unimplemented!() }

    pub fn mtable_min_label_spacing(&self) -> Option<&Length> { unimplemented!() }

    pub fn mtable_row_span(&self) -> Option<&u32> { unimplemented!() }

    pub fn mtable_column_span(&self) -> Option<&u32> { unimplemented!() }
}