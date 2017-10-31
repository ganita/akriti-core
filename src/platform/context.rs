use super::Platform;
use ::constants::{
    OperatorDictionary, 
    MathVariantsDictionary
};

pub struct Context {
    platform: Box<Platform>,
    operator_dictionary: OperatorDictionary,
    math_variants_dictionary: MathVariantsDictionary,
    font_size: f32
}

impl Context {
    pub fn new(platform: Box<Platform>, font_size: f32) -> Context {
        Context {
            platform,
            operator_dictionary: OperatorDictionary::new(),
            math_variants_dictionary: MathVariantsDictionary::new(),
            font_size
        }
    }

    pub fn platform(&self) -> &Platform {
        self.platform.as_ref()
    }

    pub fn operator_dictionary(&self) -> &OperatorDictionary {
        &self.operator_dictionary
    }

    pub fn math_variants_dictionary(&self) -> &MathVariantsDictionary {
        &self.math_variants_dictionary
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }
}