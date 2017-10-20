use ::phf;

pub use ::constants_props::MathVariant;

pub struct MathVariantsDictionary {
    dictionary: &'static phf::Map<char, phf::Map<MathVariant, char>>
}

use MathVariant::*;
include!(concat!(env!("OUT_DIR"), "/codegen_math_variant_dictionary.rs"));

impl MathVariantsDictionary {
    pub fn new() -> MathVariantsDictionary {

        MathVariantsDictionary {
            dictionary: &MATH_VARIANT_DICTIONARY
        }
    }

    pub fn variant_for_char(&self, c: &char, variant: &MathVariant) -> Option<char> {
        let variants = self.dictionary.get(c);
        if variants.is_none() {
            return None;
        }

        let variant = variants.unwrap().get(variant);

        if variant.is_none() {
            return None;
        }

        Some(*variant.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn if_variant_is_present() {
        let variants = MathVariantsDictionary::new();
        let variant = variants.variant_for_char(&'9', &MathVariant::DoubleStruck).unwrap();

        assert_eq!(variant, '𝟡');
    }

    #[test]
    fn if_variants_are_present_but_no_variant_for_specific() {
        let variants = MathVariantsDictionary::new();
        let variant = variants.variant_for_char(&'9', &MathVariant::Fraktur);

        assert_eq!(variant, None);
    }

    #[test]
    fn if_no_variants_are_present() {
        let variants = MathVariantsDictionary::new();
        let variant = variants.variant_for_char(&'√', &MathVariant::Fraktur);

        assert_eq!(variant, None);
    }
}