use super::{
    Color, 
    MathSize, 
    Directionality,
    IndentAlign, 
    Length, 
    IndentAlignFirstLast, 
    IndentShiftFirstLast, 
    VAlign,
    HAlign, 
    GroupAlign
};

#[derive(Immutable, Clone)]
pub struct InheritedProps {
    display_style: bool,
    script_level: u32,
    math_size: MathSize,
    dir: Directionality,
    math_color: Color,
    script_size_multiplier: f32,
    script_min_size: f32,
    linebreak_mult_char: String,
    indent_align:IndentAlign,
    indent_shift: Length,
    indent_target: Option<String>,
    indent_align_first: IndentAlignFirstLast,
    indent_shift_first: IndentShiftFirstLast,
    indent_align_last: IndentAlignFirstLast,
    indent_shift_last: IndentShiftFirstLast,
    table_row_align: VAlign,
    table_column_align: Vec<HAlign>,
    table_group_align: Vec<Vec<GroupAlign>>,
    table_mtd_column_align: HAlign,
    table_mtd_group_align: Vec<GroupAlign>,
}

impl InheritedProps {
    pub fn new() -> InheritedProps {
        InheritedProps {
            display_style: true,
            script_level: 1,
            math_size: MathSize::NORMAL,
            dir: Directionality::LTR,
            math_color: Color::parse("#000000").unwrap(),
            script_size_multiplier: 0.707,
            script_min_size: 8.0,
            linebreak_mult_char: String::from("\u{2062}"),
            indent_align: IndentAlign::Auto,
            indent_shift: Length::PX(0.0),
            indent_target: None,
            indent_align_first: IndentAlignFirstLast::IndentAlign,
            indent_shift_first: IndentShiftFirstLast::IndentShift,
            indent_align_last: IndentAlignFirstLast::IndentAlign,
            indent_shift_last: IndentShiftFirstLast::IndentShift,
            table_row_align: VAlign::Baseline,
            table_column_align: vec![HAlign::Center],
            table_group_align: vec![vec![GroupAlign::Left]],
            table_mtd_column_align: HAlign::Center,
            table_mtd_group_align: vec![GroupAlign::Left]
        }
    }
}