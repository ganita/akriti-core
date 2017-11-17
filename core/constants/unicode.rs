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