use ::akriti_core::paint::{TypeFace, Rect, TextRuler, MathRuler};
use ::akriti_measure::harfbuzz::HBFace;
use ::akriti_core::props::Directionality;
use ::cairo::{Context, ImageSurface, Format, FontSlant, FontWeight};

const HB_FACTOR: f32 = 64.0;


pub struct HarfbuzzMathRuler {
    hb_face: HBFace,
    context: Context,
    _surface: ImageSurface,
}

impl HarfbuzzMathRuler {
    pub fn new(face: HBFace) -> HarfbuzzMathRuler {
        let surface = ImageSurface::create(Format::ARgb32, 1024, 1024).unwrap();
        let context = Context::new(&surface);
        context.select_font_face("STIX Two Math", FontSlant::Normal, FontWeight::Normal);

        HarfbuzzMathRuler {
            hb_face: face,
            context,
            _surface: surface,
        }
    }

    pub fn set_font_size(&self, size: f32) {
        self.context.set_font_size(size as f64);
    }
}

impl TextRuler for HarfbuzzMathRuler {
    fn font_size(&self) -> f32 {
        self.context.font_extents().height as f32
    }

    fn typeface(&self) -> &TypeFace {
        unimplemented!()
    }

    fn measure(&self, text: &str, _: &Directionality) -> Rect {
        let te = self.context.text_extents(text);
        Rect::new(te.x_advance as f32, te.height as f32)
    }

    fn measure_char(&self, unicode: u32, dir: &Directionality) -> Rect {
        self.measure(&String::from_utf16(&[unicode as u16]).unwrap(), dir)
    }

    fn ascent(&self) -> f32 {
        self.context.font_extents().ascent as f32
    }

    fn descent(&self) -> f32 {
        -self.context.font_extents().descent as f32
    }
}

impl MathRuler for HarfbuzzMathRuler  {
    fn script_percent_scale_down(&self) -> f32 {
        self.hb_face.script_percent_scale_down() as f32
    }

    fn script_script_percent_scale_down(&self) -> f32 {
        self.hb_face.script_script_percent_scale_down() as f32
    }

    fn delimited_sub_formula_min_height(&self) -> f32 {
        self.hb_face.delimited_sub_formula_min_height() as f32 / HB_FACTOR
    }

    fn display_operator_min_height(&self) -> f32 {
        unimplemented!()
    }

    fn math_leading(&self) -> f32 {
        unimplemented!()
    }

    fn axis_height(&self) -> f32 {
        self.hb_face.axis_height() as f32 / HB_FACTOR
    }

    fn accent_base_height(&self) -> f32 {
        unimplemented!()
    }

    fn flattened_accent_base_height(&self) -> f32 {
        unimplemented!()
    }

    fn subscript_shift_down(&self) -> f32 {
        unimplemented!()
    }

    fn subscript_top_max(&self) -> f32 {
        unimplemented!()
    }

    fn subscript_baseline_drop_min(&self) -> f32 {
        unimplemented!()
    }

    fn subscript_shift_up(&self) -> f32 {
        unimplemented!()
    }

    fn superscript_shift_up_cramped(&self) -> f32 {
        unimplemented!()
    }

    fn superscript_bottom_min(&self) -> f32 {
        unimplemented!()
    }

    fn superscript_baseline_drop_max(&self) -> f32 {
        unimplemented!()
    }

    fn sub_superscript_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn superscript_bottom_max_with_subscript(&self) -> f32 {
        unimplemented!()
    }

    fn space_after_script(&self) -> f32 {
        unimplemented!()
    }

    fn upper_limit_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn upper_limit_baseline_rise_min(&self) -> f32 {
        unimplemented!()
    }

    fn lower_limit_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn lower_limit_baseline_drop_min(&self) -> f32 {
        unimplemented!()
    }

    fn stack_top_shift_up(&self) -> f32 {
        unimplemented!()
    }

    fn stack_top_display_style_shift_up(&self) -> f32 {
        unimplemented!()
    }

    fn stack_bottom_shift_down(&self) -> f32 {
        unimplemented!()
    }

    fn stack_bottom_display_style_shift_down(&self) -> f32 {
        unimplemented!()
    }

    fn stack_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn stack_display_style_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn stretch_stack_top_shift_up(&self) -> f32 {
        unimplemented!()
    }

    fn stretch_stack_bottom_shift_down(&self) -> f32 {
        unimplemented!()
    }

    fn stretch_stack_gap_above_min(&self) -> f32 {
        unimplemented!()
    }

    fn stretch_stack_gap_below_min(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_numerator_shift_up(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_numerator_display_style_shift_up(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_denominator_shift_down(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_denominator_display_style_shift_down(&self) -> f32 {
        unimplemented!()
    }

    fn numerator_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_num_display_style_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_rule_thickness(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_denominator_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn fraction_denominator_display_style_gap_min(&self) -> f32 {
        unimplemented!()
    }

    fn skewed_fraction_horizontal_gap(&self) -> f32 {
        unimplemented!()
    }

    fn skewed_fraction_vertical_gap(&self) -> f32 {
        unimplemented!()
    }

    fn overbar_vertical_gap(&self) -> f32 {
        unimplemented!()
    }

    fn overbar_rule_thickness(&self) -> f32 {
        unimplemented!()
    }

    fn overbar_extra_ascender(&self) -> f32 {
        unimplemented!()
    }

    fn underbar_vertical_gap(&self) -> f32 {
        unimplemented!()
    }

    fn underbar_rule_thickness(&self) -> f32 {
        unimplemented!()
    }

    fn underbar_extra_descender(&self) -> f32 {
        unimplemented!()
    }

    fn radical_vertical_gap(&self) -> f32 {
        unimplemented!()
    }

    fn radical_display_style_vertical_gap(&self) -> f32 {
        unimplemented!()
    }

    fn radical_rule_thickness(&self) -> f32 {
        unimplemented!()
    }

    fn radical_extra_ascender(&self) -> f32 {
        unimplemented!()
    }

    fn radical_kern_before_degree(&self) -> f32 {
        unimplemented!()
    }

    fn radical_kern_after_degree(&self) -> f32 {
        unimplemented!()
    }

    fn radical_degree_bottom_raise_percent(&self) -> f32 {
        unimplemented!()
    }
}