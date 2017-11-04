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


use super::typeface::TypeFace;
use super::rect::Rect;
use ::props::Directionality;

pub trait TextRuler {
    fn font_size(&self) -> f32;
    fn typeface(&self) -> &TypeFace;

    fn measure(&self, text: &str, dir: &Directionality) -> Rect;
    fn measure_char(&self, unicode: u32, dir: &Directionality) -> Rect;
    fn measure_glyph(&self, glyph_index: u32, dir: &Directionality) -> Rect;

    fn ascent(&self) -> f32;
    fn descent(&self) -> f32;
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
    fn superscript_shift_up(&self) -> f32;
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
    fn minimum_connector_overlap(&self, direction: &GlyphConstructionDirection) -> f32;
    fn glyph_variants(&self, unicode: u32, direction: &GlyphConstructionDirection) -> Vec<GlyphVariant>;
    fn glyph_assembly(&self, unicode: u32, direction: &GlyphConstructionDirection) -> GlyphAssembly;
}

pub enum GlyphConstructionDirection {
    Horizontal,
    Vertical,
}

pub struct GlyphVariant {
    glyph_index: u32,
    advance: f32,
}

impl GlyphVariant {
    pub fn new(glyph_index: u32, advance: f32) -> GlyphVariant {
        GlyphVariant { glyph_index, advance }
    }

    pub fn glyph_index(&self) -> u32 {
        self.glyph_index
    }

    pub fn advance(&self) -> f32 {
        self.advance
    }
}

pub struct GlyphAssemblyPart {
    glyph_index: u32,
    start_connector_length: f32,
    end_connector_length: f32,
    full_advance: f32,
    is_extender: bool,
}

impl GlyphAssemblyPart {
    pub fn new(glyph_index: u32, start_connector_length: f32, end_connector_length: f32,
               full_advance: f32, is_extender: bool) -> GlyphAssemblyPart {
        GlyphAssemblyPart {
            glyph_index,
            start_connector_length,
            end_connector_length,
            full_advance,
            is_extender,
        }
    }

    pub fn glyph_index(&self) -> u32 {
        self.glyph_index
    }

    pub fn start_connector_length(&self) -> f32 {
        self.start_connector_length
    }

    pub fn end_connector_length(&self) -> f32 {
        self.end_connector_length
    }

    pub fn full_advance(&self) -> f32 {
        self.full_advance
    }

    pub fn is_extender(&self) -> bool {
        self.is_extender
    }
}

pub struct GlyphAssembly {
    parts: Vec<GlyphAssemblyPart>,
    italics_correction: f32,
}

impl GlyphAssembly {
    pub fn new(parts: Vec<GlyphAssemblyPart>, italics_correction: f32) -> GlyphAssembly {
        GlyphAssembly {
            parts,
            italics_correction
        }
    }

    pub fn empty() -> GlyphAssembly {
        GlyphAssembly {
            parts: Vec::new(),
            italics_correction: 0.0,
        }
    }

    pub fn parts(&self) -> &Vec<GlyphAssemblyPart> {
        &self.parts
    }

    pub fn italics_correction(&self) -> f32 {
        self.italics_correction
    }
}