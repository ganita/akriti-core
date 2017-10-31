use std::ffi::{CString};
use std::ptr;

use ::text_ruler::CairoTextRuler;
use ::math_ruler::HarfbuzzMathRuler;
use ::freetype_sys;
use ::akriti_core::paint::{TextRuler, MathRuler};
use ::akriti_core::platform::{Platform};
use ::akriti_core::elements::Element;
use ::akriti_measure::harfbuzz::FTFontRef;
use ::akriti_measure::harfbuzz::HBFace;

pub struct GTKPlatform {
    text_ruler: CairoTextRuler,
    _ft_lib: freetype_sys::FT_Library,
    _ft_face: freetype_sys::FT_Face,
    math_ruler: HarfbuzzMathRuler
}

impl Platform for GTKPlatform {
    fn get_text_ruler(&self, _: &Element, size: f32) -> &TextRuler {
        self.text_ruler.set_size(size);
        &self.text_ruler
    }

    fn get_math_ruler(&self, _: &Element, size: f32) -> &MathRuler {
        unsafe {
            freetype_sys::FT_Set_Pixel_Sizes(self._ft_face, 0, size as u32);
        }
        self.math_ruler.set_font_size(size);
        &self.math_ruler
    }

    fn px_to_du(&self, _: f32) -> f32 {
        unimplemented!()
    }

    fn sp_to_du(&self, _: f32) -> f32 {
        unimplemented!()
    }

    fn dp_to_du(&self, _: f32) -> f32 {
        unimplemented!()
    }
}

impl GTKPlatform {
    pub fn new() -> GTKPlatform {
        let path = CString::new(format!("{}/tests/fonts/STIX2Math.otf", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let mut ft_lib = ptr::null_mut();
        unsafe {
            assert_eq!(freetype_sys::FT_Init_FreeType(&mut ft_lib), 0);
        };

        let mut ft_face = ptr::null_mut();
        unsafe {
            assert_eq!(freetype_sys::FT_New_Face(ft_lib, path.as_ptr(),
                                                 0, &mut ft_face), 0);
        };

        GTKPlatform {
            text_ruler: CairoTextRuler::new(),
            _ft_lib: ft_lib,
            _ft_face: ft_face,
            math_ruler: HarfbuzzMathRuler::new(HBFace::from_freetype_font(ft_face as FTFontRef))
        }
    }
}