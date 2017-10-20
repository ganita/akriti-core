mod context;                pub use self::context::Context;
mod display_metrics;        pub use self::display_metrics::DisplayMetrics;

use ::paint::{
    TextRuler, 
    MathRuler
};

pub trait Platform {
    fn create_text_ruler(&self) -> Box<TextRuler>;
    fn create_math_ruler(&self) -> Box<MathRuler>;
    fn display_metrics(&self) -> DisplayMetrics;
}