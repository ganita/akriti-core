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


use std::cell::RefCell;

use ::props::Directionality;
use ::paint::{
    MathRuler, TextRuler, Rect, GlyphConstructionDirection, GlyphVariant, GlyphAssembly, TypeFace,
    GlyphAssemblyPart
};
use ::skia_sys::{Paint, Typeface as SkTypeface, TextEncoding};
use ::akriti_measure::freetype::{Freetype, FreetypeFace};
use ::akriti_measure::harfbuzz::HBDirection;

pub struct Ruler {
    paint: RefCell<Paint>,
    ft_face: RefCell<FreetypeFace>,
    sk_typeface: SkTypeface,
}

impl Ruler {
    pub fn new(font_path: &str, index: u32) -> Ruler {
        let sk_typeface = SkTypeface::new_from_file(font_path, index)
            .unwrap();

        let ft_library = Freetype::new();
        let ft_face = FreetypeFace::new_from_file(ft_library, font_path, index)
            .unwrap();

        let mut paint = Paint::new();
        paint.set_typeface(&sk_typeface);

        Ruler {
            paint: RefCell::new(paint),
            ft_face: RefCell::new(ft_face),
            sk_typeface: sk_typeface
        }
    }

    pub fn set_size(&self, size: f32) {
        self.paint.borrow_mut().set_text_size(size);
        self.ft_face.borrow_mut().set_size_pixels(0, size.ceil() as u32);
    }

    pub fn get_sk_typeface(&self) -> &SkTypeface {
        &self.sk_typeface
    }
}

impl TextRuler for Ruler {
    fn font_size(&self) -> f32 {
        self.paint.borrow().get_text_size()
    }

    fn typeface(&self) -> &TypeFace {
        unimplemented!()
    }

    fn measure(&self, text: &str, _dir: &Directionality) -> Rect {
        self.paint.borrow_mut().set_text_encoding(TextEncoding::kUTF8_TextEncoding);
        let (width, _) = self.paint.borrow().measure_text(text);
        Rect::new(width, self.ascent() - self.descent())
    }

    fn measure_char(&self, unicode: u32, dir: &Directionality) -> Rect {
        self.measure(&String::from_utf16(&[unicode as u16]).unwrap(), dir)
    }

    fn measure_glyph(&self, glyph_index: u32, _: &Directionality) -> Rect {
        self.paint.borrow_mut().set_text_encoding(TextEncoding::kGlyphID_TextEncoding);
        let (width, rect) = self.paint.borrow().measure_blob(&[glyph_index as u16]);
        Rect::new(width, (rect.bottom.abs()+rect.top.abs()).max(self.ascent()-self.descent()))
    }

    fn ascent(&self) -> f32 {
        -self.paint.borrow().get_font_metrics(0.).fAscent
    }

    fn descent(&self) -> f32 {
        -self.paint.borrow().get_font_metrics(0.).fDescent
    }
}

const HB_SIZE_FACTOR: f32 = 64.;

impl MathRuler for Ruler {
    fn script_percent_scale_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.script_percent_scale_down() as f32
    }

    fn script_script_percent_scale_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.script_script_percent_scale_down() as f32
    }

    fn delimited_sub_formula_min_height(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.delimited_sub_formula_min_height() as f32 / HB_SIZE_FACTOR
    }

    fn display_operator_min_height(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.display_operator_min_height() as f32 / HB_SIZE_FACTOR
    }

    fn math_leading(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.math_leading() as f32 / HB_SIZE_FACTOR
    }

    fn axis_height(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.axis_height() as f32 / HB_SIZE_FACTOR
    }

    fn accent_base_height(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.accent_base_height() as f32 / HB_SIZE_FACTOR
    }

    fn flattened_accent_base_height(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.flattened_accent_base_height() as f32 / HB_SIZE_FACTOR
    }

    fn subscript_shift_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.subscript_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn subscript_top_max(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.subscript_top_max() as f32 / HB_SIZE_FACTOR
    }

    fn subscript_baseline_drop_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.subscript_baseline_drop_min() as f32 / HB_SIZE_FACTOR
    }

    fn superscript_shift_up(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.subscript_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn superscript_shift_up_cramped(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.subscript_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn superscript_bottom_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.superscript_bottom_min() as f32 / HB_SIZE_FACTOR
    }

    fn superscript_baseline_drop_max(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.superscript_baseline_drop_max() as f32 / HB_SIZE_FACTOR
    }

    fn sub_superscript_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.sub_superscript_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn superscript_bottom_max_with_subscript(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.superscript_bottom_max_with_subscript() as f32 / HB_SIZE_FACTOR
    }

    fn space_after_script(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.space_after_script() as f32 / HB_SIZE_FACTOR
    }

    fn upper_limit_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.sub_superscript_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn upper_limit_baseline_rise_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.upper_limit_baseline_rise_min() as f32 / HB_SIZE_FACTOR
    }

    fn lower_limit_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.lower_limit_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn lower_limit_baseline_drop_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.lower_limit_baseline_drop_min() as f32 / HB_SIZE_FACTOR
    }

    fn stack_top_shift_up(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stack_top_shift_up() as f32 / HB_SIZE_FACTOR
    }

    fn stack_top_display_style_shift_up(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stack_top_display_style_shift_up() as f32 / HB_SIZE_FACTOR
    }

    fn stack_bottom_shift_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stack_bottom_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn stack_bottom_display_style_shift_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stack_bottom_display_style_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn stack_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stack_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn stack_display_style_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stack_display_style_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn stretch_stack_top_shift_up(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stretch_stack_top_shift_up() as f32 / HB_SIZE_FACTOR
    }

    fn stretch_stack_bottom_shift_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stretch_stack_bottom_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn stretch_stack_gap_above_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stretch_stack_gap_above_min() as f32 / HB_SIZE_FACTOR
    }

    fn stretch_stack_gap_below_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.stretch_stack_gap_below_min() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_numerator_shift_up(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_numerator_shift_up() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_numerator_display_style_shift_up(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_numerator_display_style_shift_up() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_denominator_shift_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_denominator_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_denominator_display_style_shift_down(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_denominator_display_style_shift_down() as f32 / HB_SIZE_FACTOR
    }

    fn numerator_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_numerator_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_num_display_style_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_num_display_style_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_rule_thickness(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_rule_thickness() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_denominator_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_denominator_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn fraction_denominator_display_style_gap_min(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.fraction_denominator_display_style_gap_min() as f32 / HB_SIZE_FACTOR
    }

    fn skewed_fraction_horizontal_gap(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.skewed_fraction_horizontal_gap() as f32 / HB_SIZE_FACTOR
    }

    fn skewed_fraction_vertical_gap(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.skewed_fraction_vertical_gap() as f32 / HB_SIZE_FACTOR
    }

    fn overbar_vertical_gap(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.overbar_vertical_gap() as f32 / HB_SIZE_FACTOR
    }

    fn overbar_rule_thickness(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.overbar_rule_thickness() as f32 / HB_SIZE_FACTOR
    }

    fn overbar_extra_ascender(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.overbar_extra_ascender() as f32 / HB_SIZE_FACTOR
    }

    fn underbar_vertical_gap(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.underbar_vertical_gap() as f32 / HB_SIZE_FACTOR
    }

    fn underbar_rule_thickness(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.underbar_rule_thickness() as f32 / HB_SIZE_FACTOR
    }

    fn underbar_extra_descender(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.underbar_extra_descender() as f32 / HB_SIZE_FACTOR
    }

    fn radical_vertical_gap(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_vertical_gap() as f32 / HB_SIZE_FACTOR
    }

    fn radical_display_style_vertical_gap(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_display_style_vertical_gap() as f32 / HB_SIZE_FACTOR
    }

    fn radical_rule_thickness(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_rule_thickness() as f32 / HB_SIZE_FACTOR
    }

    fn radical_extra_ascender(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_extra_ascender() as f32 / HB_SIZE_FACTOR
    }

    fn radical_kern_before_degree(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_kern_before_degree() as f32 / HB_SIZE_FACTOR
    }

    fn radical_kern_after_degree(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_kern_after_degree() as f32 / HB_SIZE_FACTOR
    }

    fn radical_degree_bottom_raise_percent(&self) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();
        hb_face.radical_degree_bottom_raise_percent() as f32 / HB_SIZE_FACTOR
    }

    fn minimum_connector_overlap(&self, direction: &GlyphConstructionDirection) -> f32 {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();

        let overlap = match *direction {
            GlyphConstructionDirection::Horizontal => hb_face.min_connector_overlap_horizontal(),
            GlyphConstructionDirection::Vertical => hb_face.min_connector_overlap_vertical(),
        };

        overlap as f32 / HB_SIZE_FACTOR
    }

    fn glyph_variants(&self, unicode: u32, direction: &GlyphConstructionDirection) -> Vec<GlyphVariant> {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();

        let direction = from_glyph_construction_direction(direction);
        let glyph_index = hb_face.glyph_index(unicode);

        if glyph_index.is_none() {
            return vec![];
        }
        let glyph_index = glyph_index.unwrap();
        let variants = hb_face.glyph_variants(glyph_index, direction);

        variants
            .map(|v|
                GlyphVariant::new(v.glyph_index(), v.advance() as f32 / HB_SIZE_FACTOR))
            .collect()
    }

    fn glyph_assembly(&self, unicode: u32, direction: &GlyphConstructionDirection) -> GlyphAssembly {
        let ft_face = self.ft_face.borrow();
        let hb_face = ft_face.get_hb_face();

        let direction = from_glyph_construction_direction(direction);
        let glyph_index = hb_face.glyph_index(unicode);

        if glyph_index.is_none() {
            return GlyphAssembly::empty();
        }
        let glyph_index = glyph_index.unwrap();

        let assembly = hb_face.glyph_assembly(glyph_index, direction);

        let parts: Vec<GlyphAssemblyPart> = assembly.parts().iter().map(|p| GlyphAssemblyPart::new(
            p.glyph_index(),
            p.start_connector_length() as f32 / HB_SIZE_FACTOR,
            p.end_connector_length() as f32 / HB_SIZE_FACTOR,
            p.full_advance() as f32 / HB_SIZE_FACTOR,
            p.is_extender()
        )).collect();

        GlyphAssembly::new(parts, assembly.italics_correction() as f32 / HB_SIZE_FACTOR)
    }
}

fn from_glyph_construction_direction(direction: &GlyphConstructionDirection) -> HBDirection {
    match *direction {
        GlyphConstructionDirection::Vertical => HBDirection::TTB,
        GlyphConstructionDirection::Horizontal => HBDirection::LTR,
    }
}