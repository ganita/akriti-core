use ::cairo::{ImageSurface, Context, Format};
use ::akriti_core::paint::{TextRuler, Rect, TypeFace};
use ::akriti_core::props::Directionality;

pub struct CairoTextRuler {
    _surface: ImageSurface,
    context: Context
}

impl TextRuler for CairoTextRuler {
    fn font_size(&self) -> f32 {
        self.context.font_extents().height as f32
    }

    fn typeface(&self) -> &TypeFace {
        unimplemented!()
    }

    fn measure(&self, text: &str, _: &Directionality) -> Rect {
        let te = self.context.text_extents(text);
        Rect::new(te.width as f32, te.height as f32)
    }

    fn measure_char(&self, unicode: u32, dir: &Directionality) -> Rect {
        self.measure(&String::from_utf16(&[unicode as u16]).unwrap(), dir)
    }

    fn ascent(&self) -> f32 {
        self.context.font_extents().ascent as f32
    }

    fn descent(&self) -> f32 {
        self.context.font_extents().descent as f32
    }
}

impl CairoTextRuler {
    pub fn new() -> CairoTextRuler {
        let surface = ImageSurface::create(Format::ARgb32, 1024, 1024).unwrap();
        let context = Context::new(&surface);
        CairoTextRuler { context, _surface: surface }
    }

    pub fn set_size(&self, size: f32) {
        self.context.set_font_size(size as f64);
    }
}