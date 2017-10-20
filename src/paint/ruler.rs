use super::typeface::TypeFace;
use super::rect::Rect;

pub trait TextRuler {
    fn set_font_size(&mut self, size: f32);
    fn set_typeface(&mut self, typeface: TypeFace);
    fn font_size(&self) -> f32;
    fn typeface(&self) -> &TypeFace;

    fn measure(&self, text: &String) -> Rect;
    fn measure_char(&self, unicode: u32) -> Rect;

    fn ascent(&self) -> f32;
    fn descent(&self) -> f32;
    fn top(&self) -> f32;
    fn bottom(&self) -> f32;
}

pub trait MathRuler : TextRuler {
    fn script_percent_scale_down(&self) -> f32;
    fn script_script_percent_scale_down(&self) -> f32;
    fn delimited_sub_formula_min_height(&self) -> f32;
    fn display_operator_min_height(&self) -> f32;
    fn math_leading(&self) -> f32;
    fn axis_height(&self) -> f32;
    fn accent_base_height(&self) -> f32;
    fn flattened_accent_base_height(&self) -> f32;
    fn subscript_shift_down(&self) -> f32;
    fn subscript_top_max(&self) -> f32;
    fn subscript_baseline_drop_min(&self) -> f32;
    fn subscript_shift_up(&self) -> f32;
    fn superscript_shift_up_cramped(&self) -> f32;
    fn superscript_bottom_min(&self) -> f32;
    fn superscript_baseline_drop_max(&self) -> f32;
    fn sub_superscript_gap_min(&self) -> f32;
    fn superscript_bottom_max_with_subscript(&self) -> f32;
    fn space_after_script(&self) -> f32;
    fn upper_limit_gap_min(&self) -> f32;
    fn upper_limit_baseline_rise_min(&self) -> f32;
    fn lower_limit_gap_min(&self) -> f32;
    fn lower_limit_baseline_drop_min(&self) -> f32;
    fn stack_top_shift_up(&self) -> f32;
    fn stack_top_display_style_shift_up(&self) -> f32;
    fn stack_bottom_shift_down(&self) -> f32;
    fn stack_bottom_display_style_shift_down(&self) -> f32;
    fn stack_gap_min(&self) -> f32;
    fn stack_display_style_gap_min(&self) -> f32;
    fn stretch_stack_top_shift_up(&self) -> f32;
    fn stretch_stack_bottom_shift_down(&self) -> f32;
    fn stretch_stack_gap_above_min(&self) -> f32;
    fn stretch_stack_gap_below_min(&self) -> f32;
    fn fraction_numerator_shift_up(&self) -> f32;
    fn fraction_numerator_display_style_shift_up(&self) -> f32;
    fn fraction_denominator_shift_down(&self) -> f32;
    fn fraction_denominator_display_style_shift_down(&self) -> f32;
    fn numerator_gap_min(&self) -> f32;
    fn fraction_num_display_style_gap_min(&self) -> f32;
    fn fraction_rule_thickness(&self) -> f32;
    fn fraction_denominator_gap_min(&self) -> f32;
    fn fraction_denominator_display_style_gap_min(&self) -> f32;
    fn skewed_fraction_horizontal_gap(&self) -> f32;
    fn skewed_fraction_vertical_gap(&self) -> f32;
    fn overbar_vertical_gap(&self) -> f32;
    fn overbar_rule_thickness(&self) -> f32;
    fn overbar_extra_ascender(&self) -> f32;
    fn underbar_vertical_gap(&self) -> f32;
    fn underbar_rule_thickness(&self) -> f32;
    fn underbar_extra_descender(&self) -> f32;
    fn radical_vertical_gap(&self) -> f32;
    fn radical_display_style_vertical_gap(&self) -> f32;
    fn radical_rule_thickness(&self) -> f32;
    fn radical_extra_ascender(&self) -> f32;
    fn radical_kern_before_degree(&self) -> f32;
    fn radical_kern_after_degree(&self) -> f32;
    fn radical_degree_bottom_raise_percent(&self) -> f32;
}