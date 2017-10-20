use super::{
    Color,
    MathVariant, 
    MathSize, 
    LineBreak, 
    Length, 
    HAlign, 
    LineThickness, 
    PseudoLength,
    PropertyModifier, 
    Notation, 
    Accent, 
    TableVAlign, 
    VAlign, 
    GroupAlign, 
    LineType,
    FrameSpacing, 
    TableSide
};

#[derive(Clone, Default)]
pub struct StyleProps {
    math_background: Option<Color>,
    math_variant: Option<MathVariant>,
    math_size: Option<MathSize>,
    linebreak: Option<LineBreak>,
    line_leading: Option<Length>,

    mspace_width: Option<Length>,
    mspace_height: Option<Length>,
    mspace_depth: Option<Length>,

    ms_lquote: Option<String>,
    ms_rquote: Option<String>,

    frac_line_thickness: Option<LineThickness>,
    frac_num_align: Option<HAlign>,
    frac_denom_align: Option<HAlign>,
    frac_bevelled: Option<bool>,

    mpadded_height: Option<PropertyModifier<PseudoLength>>,
    mpadded_width: Option<PropertyModifier<PseudoLength>>,
    mpadded_depth: Option<PropertyModifier<PseudoLength>>,
    mpadded_lspace: Option<PropertyModifier<PseudoLength>>,
    mpadded_voffset: Option<PropertyModifier<PseudoLength>>,

    mfence_open: Option<String>,
    mfence_close: Option<String>,
    mfence_separators: Option<String>,

    menclose_notation: Option<Vec<Notation>>,

    subscript_shift: Option<Length>,
    superscript_shift: Option<Length>,

    script_accent_under: Option<Accent>,
    script_align: Option<HAlign>,
    script_accent_over: Option<Accent>,

    table_align: Option<TableVAlign>,
    table_row_align: Option<Vec<VAlign>>,
    table_column_align: Option<Vec<HAlign>>,
    table_group_align: Option<Vec<Vec<GroupAlign>>>,
    table_alignment_scope: Option<Vec<bool>>,
    table_column_width: Option<Vec<Length>>,
    table_width: Option<Length>,
    table_row_spacing: Option<Vec<Length>>,
    table_column_spacing: Option<Vec<Length>>,
    table_row_lines: Option<Vec<LineType>>,
    table_column_lines: Option<Vec<LineType>>,
    table_frame: Option<LineType>,
    table_frame_spacing: Option<FrameSpacing>,
    table_equal_rows: Option<bool>,
    table_equal_columns: Option<bool>,
    table_display_style: Option<bool>,
    table_side: Option<TableSide>,
    table_min_label_spacing: Option<Length>,

    table_row_span: Option<u32>,
    table_column_span: Option<u32>,
}

impl StyleProps {
    pub fn new() -> StyleProps {
        StyleProps::default()
    }
}